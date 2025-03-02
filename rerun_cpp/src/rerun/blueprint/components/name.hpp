// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/components/name.fbs".

#pragma once

#include "../../result.hpp"

#include <cstdint>
#include <memory>
#include <string>
#include <utility>

namespace arrow {
    class Array;
    class DataType;
    class StringBuilder;
} // namespace arrow

namespace rerun::blueprint::components {
    /// **Component**: The name of a blueprint entity.
    struct Name {
        std::string value;

      public:
        Name() = default;

        Name(std::string value_) : value(std::move(value_)) {}

        Name& operator=(std::string value_) {
            value = std::move(value_);
            return *this;
        }
    };
} // namespace rerun::blueprint::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<blueprint::components::Name> {
        static constexpr const char Name[] = "rerun.blueprint.components.Name";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::StringBuilder* builder, const blueprint::components::Name* elements,
            size_t num_elements
        );

        /// Serializes an array of `rerun::blueprint:: components::Name` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::Name* instances, size_t num_instances
        );
    };
} // namespace rerun
