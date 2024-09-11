// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/video_timestamp.fbs".

#pragma once

#include "../datatypes/video_timestamp.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>

namespace rerun::components {
    /// **Component**: Timestamp inside a `archetypes::AssetVideo`.
    ///
    /// ⚠ **This is an experimental API! It is not fully supported, and is likely to change significantly in future versions.**
    struct VideoTimestamp {
        rerun::datatypes::VideoTimestamp timestamp;

      public:
        VideoTimestamp() = default;

        VideoTimestamp(rerun::datatypes::VideoTimestamp timestamp_) : timestamp(timestamp_) {}

        VideoTimestamp& operator=(rerun::datatypes::VideoTimestamp timestamp_) {
            timestamp = timestamp_;
            return *this;
        }

        /// Cast to the underlying VideoTimestamp datatype
        operator rerun::datatypes::VideoTimestamp() const {
            return timestamp;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::VideoTimestamp) == sizeof(components::VideoTimestamp));

    /// \private
    template <>
    struct Loggable<components::VideoTimestamp> {
        static constexpr const char Name[] = "rerun.components.VideoTimestamp";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::VideoTimestamp>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::VideoTimestamp` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::VideoTimestamp* instances, size_t num_instances
        ) {
            return Loggable<rerun::datatypes::VideoTimestamp>::to_arrow(
                &instances->timestamp,
                num_instances
            );
        }
    };
} // namespace rerun
