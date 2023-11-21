// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

#include "affix_fuzzer15.hpp"

#include "../datatypes/affix_fuzzer3.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::components {
    const char AffixFuzzer15::NAME[] = "rerun.testing.components.AffixFuzzer15";

    const std::shared_ptr<arrow::DataType>& AffixFuzzer15::arrow_datatype() {
        static const auto datatype = rerun::datatypes::AffixFuzzer3::arrow_datatype();
        return datatype;
    }

    rerun::Error AffixFuzzer15::fill_arrow_array_builder(
        arrow::DenseUnionBuilder* builder, const AffixFuzzer15* elements, size_t num_elements
    ) {
        (void)builder;
        (void)elements;
        (void)num_elements;
        if (true) {
            return rerun::Error(
                ErrorCode::NotImplemented,
                "TODO(andreas) Handle nullable extensions"
            );
        }

        return Error::ok();
    }

    Result<rerun::DataCell> AffixFuzzer15::to_data_cell(
        const AffixFuzzer15* instances, size_t num_instances
    ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(arrow_datatype(), pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(AffixFuzzer15::fill_arrow_array_builder(
                static_cast<arrow::DenseUnionBuilder*>(builder.get()),
                instances,
                num_instances
            ));
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));

        DataCell cell;
        cell.num_instances = num_instances;
        cell.component_name = AffixFuzzer15::NAME;
        cell.array = std::move(array);
        return cell;
    }
} // namespace rerun::components
