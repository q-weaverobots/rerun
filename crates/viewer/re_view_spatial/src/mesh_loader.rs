use itertools::Itertools;
use re_chunk_store::RowId;
use re_renderer::{mesh::GpuMesh, RenderContext, Rgba32Unmul};
use re_types::components::MediaType;
use re_viewer_context::{gpu_bridge::texture_creation_desc_from_color_image, ImageInfo};

use crate::{mesh_cache::AnyMesh, visualizers::entity_iterator::clamped_vec_or};

#[derive(Debug, Clone)]
pub struct NativeAsset3D<'a> {
    pub bytes: &'a [u8],
    pub media_type: Option<MediaType>,
    pub albedo_factor: Option<re_renderer::Rgba>,
}

#[derive(Debug, Clone)]
pub struct NativeMesh3D<'a> {
    pub vertex_positions: &'a [glam::Vec3],
    pub vertex_normals: Option<&'a [glam::Vec3]>,
    pub vertex_colors: Option<&'a [re_renderer::Rgba32Unmul]>,
    pub vertex_texcoords: Option<&'a [glam::Vec2]>,

    pub triangle_indices: Option<&'a [glam::UVec3]>,

    pub albedo_factor: Option<re_renderer::Color32>,

    pub albedo_texture_buffer: Option<re_types::datatypes::Blob>,
    pub albedo_texture_format: Option<re_types::datatypes::ImageFormat>,
}

pub struct LoadedMesh {
    name: String,

    // TODO(andreas): We should only have MeshHandles here (which are generated by the MeshManager!)
    // Can't do that right now because it's too hard to pass the render context through.
    pub mesh_instances: Vec<re_renderer::renderer::GpuMeshInstance>,

    bbox: re_math::BoundingBox,
}

impl LoadedMesh {
    pub fn load(
        name: String,
        mesh: AnyMesh<'_>,
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        // TODO(emilk): load CpuMesh in background thread.
        match mesh {
            AnyMesh::Asset { asset } => Ok(Self::load_asset3d(name, asset, render_ctx)?),
            AnyMesh::Mesh { mesh, texture_key } => {
                Ok(Self::load_mesh3d(name, mesh, texture_key, render_ctx)?)
            }
        }
    }

    fn load_asset3d(
        name: String,
        asset: NativeAsset3D<'_>,
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        re_tracing::profile_function!();

        let NativeAsset3D {
            bytes,
            media_type,
            albedo_factor,
        } = asset;

        let media_type = MediaType::or_guess_from_data(media_type, bytes)
            .ok_or_else(|| anyhow::anyhow!("couldn't guess media type"))?;

        let mut cpu_model = match media_type.as_str() {
            MediaType::GLTF | MediaType::GLB => {
                re_renderer::importer::gltf::load_gltf_from_buffer(&name, bytes, render_ctx)?
            }
            MediaType::OBJ => re_renderer::importer::obj::load_obj_from_buffer(bytes, render_ctx)?,
            MediaType::STL => re_renderer::importer::stl::load_stl_from_buffer(bytes, render_ctx)?,
            _ => anyhow::bail!("{media_type} files are not supported"),
        };

        // Overwriting albedo_factor of CpuMesh if specified in the Asset3D
        if let Some(albedo_factor) = albedo_factor {
            for instance in &cpu_model.instances {
                for material in &mut cpu_model.meshes[instance.mesh].materials {
                    material.albedo_factor = albedo_factor;
                }
            }
        }

        let bbox = cpu_model.calculate_bounding_box();
        let mesh_instances = cpu_model.into_gpu_meshes(render_ctx)?;

        Ok(Self {
            name,
            bbox,
            mesh_instances,
        })
    }

    fn load_mesh3d(
        name: String,
        mesh3d: NativeMesh3D<'_>,
        texture_key: u64,
        render_ctx: &RenderContext,
    ) -> anyhow::Result<Self> {
        re_tracing::profile_function!();

        let NativeMesh3D {
            vertex_positions,
            vertex_normals,
            vertex_colors,
            vertex_texcoords,
            triangle_indices,
            albedo_factor,
            albedo_texture_buffer,
            albedo_texture_format,
        } = mesh3d;

        let num_positions = vertex_positions.len();

        let triangle_indices = if let Some(triangle_indices) = triangle_indices {
            re_tracing::profile_scope!("copy_indices");
            triangle_indices.to_vec()
        } else {
            re_tracing::profile_scope!("generate_indices");
            anyhow::ensure!(num_positions % 3 == 0);
            (0..num_positions as u32)
                .tuples::<(_, _, _)>()
                .map(glam::UVec3::from)
                .collect::<Vec<_>>()
        };
        let num_indices = triangle_indices.len() * 3;

        let vertex_colors = if let Some(vertex_colors) = vertex_colors {
            re_tracing::profile_scope!("copy_colors");
            clamped_vec_or(vertex_colors, num_positions, &Rgba32Unmul::WHITE)
        } else {
            vec![Rgba32Unmul::WHITE; num_positions]
        };

        let vertex_normals = if let Some(normals) = vertex_normals {
            re_tracing::profile_scope!("collect_normals");
            clamped_vec_or(normals, num_positions, &glam::Vec3::ZERO)
        } else {
            // TODO(andreas): Calculate normals
            vec![glam::Vec3::ZERO; num_positions]
        };

        let vertex_texcoords = if let Some(texcoords) = vertex_texcoords {
            re_tracing::profile_scope!("collect_texcoords");
            clamped_vec_or(texcoords, num_positions, &glam::Vec2::ZERO)
        } else {
            vec![glam::Vec2::ZERO; num_positions]
        };

        let bbox = {
            re_tracing::profile_scope!("bbox");
            re_math::BoundingBox::from_points(vertex_positions.iter().copied())
        };

        let albedo = try_get_or_create_albedo_texture(
            &albedo_texture_buffer,
            &albedo_texture_format,
            render_ctx,
            texture_key,
            &name,
        )
        .unwrap_or_else(|| {
            render_ctx
                .texture_manager_2d
                .white_texture_unorm_handle()
                .clone()
        });

        let mesh = re_renderer::mesh::CpuMesh {
            label: name.clone().into(),
            triangle_indices,
            vertex_positions: vertex_positions.into(),
            vertex_colors,
            vertex_normals,
            vertex_texcoords,
            materials: smallvec::smallvec![re_renderer::mesh::Material {
                label: name.clone().into(),
                index_range: 0..num_indices as _,
                albedo,
                albedo_factor: albedo_factor.unwrap_or(re_renderer::Color32::WHITE).into(),
            }],
        };

        let mesh_instances = vec![re_renderer::renderer::GpuMeshInstance::new(
            std::sync::Arc::new(GpuMesh::new(render_ctx, &mesh)?),
        )];

        Ok(Self {
            name,
            bbox,
            mesh_instances,
        })
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bbox(&self) -> re_math::BoundingBox {
        self.bbox
    }
}

fn try_get_or_create_albedo_texture(
    albedo_texture_buffer: &Option<re_types::datatypes::Blob>,
    albedo_texture_format: &Option<re_types::datatypes::ImageFormat>,
    render_ctx: &RenderContext,
    texture_key: u64,
    name: &str,
) -> Option<re_renderer::resource_managers::GpuTexture2D> {
    let (Some(albedo_texture_buffer), Some(albedo_texture_format)) =
        (albedo_texture_buffer, albedo_texture_format)
    else {
        return None;
    };

    re_tracing::profile_function!();

    let image_info = ImageInfo {
        buffer_row_id: RowId::ZERO,            // unused
        buffer: albedo_texture_buffer.clone(), // shallow clone
        format: *albedo_texture_format,
        kind: re_types::image::ImageKind::Color,
    };

    if re_viewer_context::gpu_bridge::required_shader_decode(
        render_ctx.device_caps(),
        albedo_texture_format,
    )
    .is_some()
    {
        re_log::warn_once!("Mesh can't yet handle encoded image formats like NV12 & YUY2 or BGR(A) formats without a channel type other than U8. Ignoring the texture at {name:?}.");
        return None;
    }

    let texture =
        re_viewer_context::gpu_bridge::get_or_create_texture(render_ctx, texture_key, || {
            let debug_name = "mesh albedo texture";
            texture_creation_desc_from_color_image(
                render_ctx.device_caps(),
                &image_info,
                debug_name,
            )
        });

    match texture {
        Ok(texture) => Some(texture),
        Err(err) => {
            re_log::warn_once!("Failed to create mesh albedo texture for {name:?}: {err}");
            None
        }
    }
}
