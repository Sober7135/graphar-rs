#include "graphar_rs.h"

#include <cassert>
#include <cstddef>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

namespace graphar_rs {
std::shared_ptr<graphar::InfoVersion> new_info_version(int version) {
  return std::make_shared<graphar::InfoVersion>(version);
}

std::shared_ptr<const graphar::InfoVersion>
new_const_info_version(int version) {
  return std::make_shared<graphar::InfoVersion>(version);
}

std::shared_ptr<graphar::GraphInfo> load_graph_info(const std::string &path) {
  return graphar::GraphInfo::Load(path).value();
}

void graph_info_save(const graphar::GraphInfo &graph_info,
                     const std::string &path) {
  auto status = graph_info.Save(path);
  assert(status.ok());
}
std::unique_ptr<std::string>
graph_info_dump(const graphar::GraphInfo &graph_info) {
  return std::make_unique<std::string>(graph_info.Dump().value());
}

std::unique_ptr<std::vector<graphar::Property>> new_properties() {
  return std::make_unique<std::vector<graphar::Property>>();
}
void push_property(std::vector<graphar::Property> &properties,
                   const std::string &name,
                   const std::shared_ptr<graphar::DataType> &type,
                   bool is_primary, bool is_nullable,
                   graphar::Cardinality cardinality) {
  // auto property =
  //     graphar::Property(name, type, is_primary, is_nullable, cardinality);
  properties.emplace_back(name, type, is_primary, is_nullable, cardinality);
}

std::unique_ptr<graphar::PropertyGroupVector> new_property_group_vec() {
  return std::make_unique<graphar::PropertyGroupVector>();
}
void push_property_group(
    graphar::PropertyGroupVector &vec,
    std::shared_ptr<graphar::PropertyGroup> property_group) {
  vec.emplace_back(property_group);
}

// TODO(exception)
void vertex_info_save(const graphar::VertexInfo &v, const std::string &path) {
  auto status = v.Save(path);
  assert(status.ok());
}

std::unique_ptr<std::string> vertex_info_dump(const graphar::VertexInfo &v) {
  return std::make_unique<std::string>(v.Dump().value());
}

std::shared_ptr<graphar::VertexInfo>
create_vertex_info(const rust::String &type, graphar::IdType chunk_size,
                   const graphar::PropertyGroupVector &property_group,
                   const rust::Vec<rust::String> &labels,
                   const rust::String &prefix,
                   std::shared_ptr<const graphar::InfoVersion> version) {
  std::vector<std::string> label_vec;
  label_vec.reserve(labels.size());
  for (size_t i; i < labels.size(); ++i) {
    label_vec.emplace_back(std::string(labels[i]));
  }

  return graphar::CreateVertexInfo(std::string(type), chunk_size,
                                   property_group, label_vec,
                                   std::string(prefix), version);
}

std::unique_ptr<graphar::builder::Vertex> new_vertex() {
  return std::make_unique<graphar::builder::Vertex>();
}

// Vertex add property
void vertex_add_property_bool(graphar::builder::Vertex &v,
                              const std::string &name, bool val) {
  v.AddProperty(name, val);
}
void vertex_add_property_i32(graphar::builder::Vertex &v,
                             const std::string &name, int32_t val) {
  v.AddProperty(name, val);
}
void vertex_add_property_i64(graphar::builder::Vertex &v,
                             const std::string &name, int64_t val) {
  v.AddProperty(name, val);
}
void vertex_add_property_f32(graphar::builder::Vertex &v,
                             const std::string &name, float val) {
  v.AddProperty(name, val);
}
void vertex_add_property_f64(graphar::builder::Vertex &v,
                             const std::string &name, double val) {
  v.AddProperty(name, val);
}
void vertex_add_property_string(graphar::builder::Vertex &v,
                                const std::string &name,
                                const std::string &val) {
  v.AddProperty(name, val);
}

std::shared_ptr<graphar::builder::VerticesBuilder>
new_vertices_builder(const std::shared_ptr<graphar::VertexInfo> &vertex_info,
                     const std::string &path_prefix,
                     graphar::IdType start_index) {
  return graphar::builder::VerticesBuilder::Make(vertex_info, path_prefix,
                                                 start_index)
      .value();
}

// TODO(exception)
void add_vertex(graphar::builder::VerticesBuilder &builder,
                graphar::builder::Vertex &v) {
  auto status = builder.AddVertex(v);
  assert(status.ok());
}

// TODO(exception)
void vertices_dump(graphar::builder::VerticesBuilder &builder) {
  auto status = builder.Dump();
  std::cout << status.message() << std::endl;
  assert(status.ok());
}

std::unique_ptr<graphar::AdjacentListVector> new_adjacent_list_vec() {
  return std::make_unique<graphar::AdjacentListVector>();
}
void push_adjacent_list(graphar::AdjacentListVector &v,
                        std::shared_ptr<graphar::AdjacentList> adj_list) {
  v.emplace_back(adj_list);
}

void edge_info_save(const graphar::EdgeInfo &edge_info,
                    const std::string &path) {
  auto status = edge_info.Save(path);
  assert(status.ok());
}
std::unique_ptr<std::string>
edge_info_dump(const graphar::EdgeInfo &edge_info) {
  return std::make_unique<std::string>(edge_info.Dump().value());
}

std::unique_ptr<graphar::builder::Edge> new_edge(graphar::IdType src_id,
                                                 graphar::IdType dst_id) {
  return std::make_unique<graphar::builder::Edge>(src_id, dst_id);
}
// edge add property
void edge_add_property_bool(graphar::builder::Edge &e, const std::string &name,
                            bool val) {
  e.AddProperty(name, val);
}
void edge_add_property_i32(graphar::builder::Edge &e, const std::string &name,
                           int32_t val) {
  e.AddProperty(name, val);
}
void edge_add_property_i64(graphar::builder::Edge &e, const std::string &name,
                           int64_t val) {
  e.AddProperty(name, val);
}
void edge_add_property_f32(graphar::builder::Edge &e, const std::string &name,
                           float val) {
  e.AddProperty(name, val);
}
void edge_add_property_f64(graphar::builder::Edge &e, const std::string &name,
                           double val) {
  e.AddProperty(name, val);
}
void edge_add_property_string(graphar::builder::Edge &e,
                              const std::string &name, const std::string &val) {
  e.AddProperty(name, val);
}

std::shared_ptr<graphar::builder::EdgesBuilder>
new_edges_builder(const std::shared_ptr<graphar::EdgeInfo> &edge_info,
                  const std::string &path_prefix,
                  graphar::AdjListType adj_list_type,
                  graphar::IdType vertices_num) {
  return graphar::builder::EdgesBuilder::Make(edge_info, path_prefix,
                                              adj_list_type, vertices_num)
      .value();
}

// TODO(exception)
void add_edge(graphar::builder::EdgesBuilder &builder,
              graphar::builder::Edge &e) {
  auto status = builder.AddEdge(e);
  assert(status.ok());
}

// TODO(exception)
void edges_dump(graphar::builder::EdgesBuilder &builder) {
  auto status = builder.Dump();
  assert(status.ok());
}

} // namespace graphar_rs