#pragma once

#include <arrow/array/array_base.h>
#include <catch2/catch_test_macros.hpp>

#include <rerun/as_components.hpp>
#include <rerun/collection.hpp>
#include <rerun/component_batch.hpp>

template <typename T>
void test_compare_archetype_serialization(const T& arch_a, const T& arch_b) {
    THEN("convert to component lists") {
        auto arch_b_serialized_result = rerun::AsComponents<T>::serialize(arch_b);
        auto arch_a_serialized_result = rerun::AsComponents<T>::serialize(arch_a);

        AND_THEN("serializing each list succeeds") {
            REQUIRE(arch_b_serialized_result.is_ok());
            REQUIRE(arch_a_serialized_result.is_ok());

            const auto& arch_b_serialized = arch_b_serialized_result.value;
            const auto& arch_a_serialized = arch_a_serialized_result.value;
            REQUIRE(arch_b_serialized.size() == arch_a_serialized.size());

            AND_THEN("the serialized data is the same") {
                for (size_t i = 0; i < arch_b_serialized.size(); ++i) {
                    INFO("Component batch #" << i);
                    CHECK(
                        arch_a_serialized[i].component_type == arch_b_serialized[i].component_type
                    );
                    INFO(
                        "Array diff: "
                        << arch_a_serialized[i].array->Diff(*arch_b_serialized[i].array)
                    );
                    CHECK(arch_b_serialized[i].array->Equals(*arch_a_serialized[i].array));
                }
            }
        }
    }
}
