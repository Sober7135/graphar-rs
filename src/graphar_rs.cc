#include "graphar_rs.h"
#include "graphar/fwd.h"
#include "graphar/high-level/graph_reader.h"

#include <algorithm>
#include <cassert>
#include <cstddef>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

// Should we replace std::runtime_error with custom exception??
namespace graphar_rs {

namespace {

template <typename T> T ValueOrThrow(graphar::Result<T> &&result) {
  if (result.has_error()) {
    throw std::runtime_error(result.error().message());
  }
  return std::move(result).value();
}

inline void ThrowIfStatusError(const graphar::Status &status) {
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}

} // namespace

std::shared_ptr<graphar::InfoVersion> new_info_version(int version) {
  return std::make_shared<graphar::InfoVersion>(version);
}

std::shared_ptr<const graphar::InfoVersion>
new_const_info_version(int version) {
  return std::make_shared<graphar::InfoVersion>(version);
}

std::shared_ptr<graphar::GraphInfo> load_graph_info(const std::string &path) {
  auto r = graphar::GraphInfo::Load(path);
  if (!r) {
    throw std::runtime_error(r.error().message());
  }
  return std::move(r).value();
}

void graph_info_save(const graphar::GraphInfo &graph_info,
                     const std::string &path) {
  auto status = graph_info.Save(path);
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}
std::unique_ptr<std::string>
graph_info_dump(const graphar::GraphInfo &graph_info) {
  auto r = graph_info.Dump();
  if (!r) {
    throw std::runtime_error(r.error().message());
  }
  return std::make_unique<std::string>(std::move(r).value());
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

void vertex_info_save(const graphar::VertexInfo &v, const std::string &path) {
  auto status = v.Save(path);
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}

std::unique_ptr<std::string> vertex_info_dump(const graphar::VertexInfo &v) {
  auto r = v.Dump();
  if (!r) {
    throw std::runtime_error(r.error().message());
  }
  return std::make_unique<std::string>(std::move(r).value());
}

std::shared_ptr<graphar::VertexInfo>
create_vertex_info(const rust::String &type, graphar::IdType chunk_size,
                   const graphar::PropertyGroupVector &property_group,
                   const rust::Vec<rust::String> &labels,
                   const rust::String &prefix,
                   std::shared_ptr<const graphar::InfoVersion> version) {
  std::vector<std::string> label_vec;
  label_vec.reserve(labels.size());
  for (size_t i = 0; i < labels.size(); ++i) {
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
  auto r = graphar::builder::VerticesBuilder::Make(vertex_info, path_prefix,
                                                   start_index);
  if (!r) {
    throw std::runtime_error(r.error().message());
  }
  return std::move(r).value();
}

void add_vertex(graphar::builder::VerticesBuilder &builder,
                graphar::builder::Vertex &v) {
  auto status = builder.AddVertex(v);
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}

void vertices_dump(graphar::builder::VerticesBuilder &builder) {
  auto status = builder.Dump();
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
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
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}
std::unique_ptr<std::string>
edge_info_dump(const graphar::EdgeInfo &edge_info) {
  auto r = edge_info.Dump();
  if (!r) {
    throw std::runtime_error(r.error().message());
  }
  return std::make_unique<std::string>(std::move(r).value());
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
  auto r = graphar::builder::EdgesBuilder::Make(edge_info, path_prefix,
                                                adj_list_type, vertices_num);
  if (!r) {
    throw std::runtime_error(r.error().message());
  }
  return std::move(r).value();
}

void add_edge(graphar::builder::EdgesBuilder &builder,
              graphar::builder::Edge &e) {
  auto status = builder.AddEdge(e);
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}

void edges_dump(graphar::builder::EdgesBuilder &builder) {
  auto status = builder.Dump();
  if (!status.ok()) {
    throw std::runtime_error(status.message());
  }
}

// VertexIter
graphar::IdType vertex_iter_id(graphar::VertexIter &iter) { return iter.id(); }

namespace {

template <typename T>
T VertexPropertyOrThrow(graphar::VertexIter &iter, const std::string &name) {
  auto vertex = (*iter);
  auto result = vertex.property<T>(name);
  return ValueOrThrow(std::move(result));
}

} // namespace

bool vertex_iter_property_bool(graphar::VertexIter &iter,
                               const std::string &name) {
  return VertexPropertyOrThrow<bool>(iter, name);
}

int32_t vertex_iter_property_i32(graphar::VertexIter &iter,
                                 const std::string &name) {
  return VertexPropertyOrThrow<int32_t>(iter, name);
}

int64_t vertex_iter_property_i64(graphar::VertexIter &iter,
                                 const std::string &name) {
  return VertexPropertyOrThrow<int64_t>(iter, name);
}

float vertex_iter_property_f32(graphar::VertexIter &iter,
                               const std::string &name) {
  return VertexPropertyOrThrow<float>(iter, name);
}

double vertex_iter_property_f64(graphar::VertexIter &iter,
                                const std::string &name) {
  return VertexPropertyOrThrow<double>(iter, name);
}

rust::String vertex_iter_property_string(graphar::VertexIter &iter,
                                         const std::string &name) {
  return rust::String(VertexPropertyOrThrow<std::string>(iter, name));
}

bool vertex_iter_has_label(graphar::VertexIter &iter,
                           const std::string &label) {
  auto result = iter.hasLabel(label);
  return ValueOrThrow(std::move(result));
}

std::unique_ptr<std::vector<std::string>>
vertex_iter_labels(graphar::VertexIter &iter) {
  auto result = iter.label();
  auto labels = ValueOrThrow(std::move(result));
  return std::make_unique<std::vector<std::string>>(labels);
}

void vertex_iter_next(graphar::VertexIter &iter) { ++iter; }
namespace {

template <typename T>
T EdgePropertyOrThrow(graphar::EdgeIter &iter, const std::string &name) {
  auto edge = (*iter);
  auto result = edge.property<T>(name);
  return ValueOrThrow(std::move(result));
}

} // namespace

bool edge_iter_property_bool(graphar::EdgeIter &iter, const std::string &name) {
  return EdgePropertyOrThrow<bool>(iter, name);
}

int32_t edge_iter_property_i32(graphar::EdgeIter &iter,
                               const std::string &name) {
  return EdgePropertyOrThrow<int32_t>(iter, name);
}

int64_t edge_iter_property_i64(graphar::EdgeIter &iter,
                               const std::string &name) {
  return EdgePropertyOrThrow<int64_t>(iter, name);
}

float edge_iter_property_f32(graphar::EdgeIter &iter, const std::string &name) {
  return EdgePropertyOrThrow<float>(iter, name);
}

double edge_iter_property_f64(graphar::EdgeIter &iter,
                              const std::string &name) {
  return EdgePropertyOrThrow<double>(iter, name);
}

rust::String edge_iter_property_string(graphar::EdgeIter &iter,
                                       const std::string &name) {
  return rust::String(EdgePropertyOrThrow<std::string>(iter, name));
}

void edge_iter_next(graphar::EdgeIter &iter) { ++iter; }

bool edge_iter_next_src(graphar::EdgeIter &iter) { return iter.next_src(); }

bool edge_iter_next_dst(graphar::EdgeIter &iter) { return iter.next_dst(); }

bool edge_iter_next_src_with_id(graphar::EdgeIter &iter, graphar::IdType id) {
  return iter.next_src(id);
}

bool edge_iter_next_dst_with_id(graphar::EdgeIter &iter, graphar::IdType id) {
  return iter.next_dst(id);
}

// VerticesCollection helpers
std::shared_ptr<graphar::VerticesCollection>
vertices_collection_make(const std::shared_ptr<graphar::GraphInfo> &graph_info,
                         const std::string &type) {
  return ValueOrThrow(graphar::VerticesCollection::Make(graph_info, type));
}

std::unique_ptr<graphar::VertexIter>
vertices_collection_begin(graphar::VerticesCollection &collection) {
  return std::make_unique<graphar::VertexIter>(collection.begin());
}

std::unique_ptr<graphar::VertexIter>
vertices_collection_end(graphar::VerticesCollection &vc) {
  return std::make_unique<graphar::VertexIter>(vc.end());
}

std::unique_ptr<graphar::VertexIter>
vertices_collection_find(graphar::VerticesCollection &vc, graphar::IdType id) {
  return std::make_unique<graphar::VertexIter>(vc.find(id));
}

std::unique_ptr<std::vector<graphar::IdType>>
filter_by_label_with_chunk(graphar::VerticesCollection &vc,
                           const std::vector<std::string> &filter_labels,
                           std::vector<graphar::IdType> &new_vaild_chunk) {
  auto res = ValueOrThrow<std::vector<graphar::IdType>>(
      vc.filter(filter_labels, &new_vaild_chunk));
  return std::make_unique<std::vector<graphar::IdType>>(res);
}

std::unique_ptr<std::vector<graphar::IdType>>
filter_by_label(graphar::VerticesCollection &vc,
                const std::vector<std::string> &filter_labels) {
  auto res = ValueOrThrow(vc.filter(filter_labels));
  return std::make_unique<std::vector<graphar::IdType>>(res);
}

std::unique_ptr<std::vector<graphar::IdType>>
filter_by_acero(const graphar::VerticesCollection &vc,
                const std::vector<std::string> &filter_labels) {
  auto res = ValueOrThrow(vc.filter_by_acero(filter_labels));
  return std::make_unique<std::vector<graphar::IdType>>(res);
}

std::unique_ptr<std::vector<graphar::IdType>>
filter_by_property_name_with_chunk(
    graphar::VerticesCollection &vc, const std::string &property_name,
    std::shared_ptr<graphar::Expression> filter_expr,
    std::vector<graphar::IdType> &new_vaild_chunk) {
  auto res =
      ValueOrThrow(vc.filter(property_name, filter_expr, &new_vaild_chunk));
  return std::make_unique<std::vector<graphar::IdType>>(res);
}

std::unique_ptr<std::vector<graphar::IdType>>
filter_by_property_name(graphar::VerticesCollection &vc,
                        const std::string &property_name,
                        std::shared_ptr<graphar::Expression> filter_expr) {
  auto res = ValueOrThrow(vc.filter(property_name, filter_expr));
  return std::make_unique<std::vector<graphar::IdType>>(res);
}

std::shared_ptr<graphar::VerticesCollection> vertices_collection_with_label(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &type, const std::string &label) {
  return ValueOrThrow(
      graphar::VerticesCollection::verticesWithLabel(label, graph_info, type));
}

std::shared_ptr<graphar::VerticesCollection> vertices_collection_with_labels(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &type, const std::vector<std::string> &labels) {
  return ValueOrThrow(graphar::VerticesCollection::verticesWithMultipleLabels(
      labels, graph_info, type));
}

std::shared_ptr<graphar::VerticesCollection> vertices_collection_with_property(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &type, const std::string &property_name,
    const std::shared_ptr<graphar::Expression> &filter) {
  return ValueOrThrow(graphar::VerticesCollection::verticesWithProperty(
      property_name, filter, graph_info, type));
}

// EdgesCollection helpers
std::unique_ptr<graphar::EdgeIter>
edges_collection_begin(graphar::EdgesCollection &collection) {
  return std::make_unique<graphar::EdgeIter>(collection.begin());
}

std::unique_ptr<graphar::EdgeIter>
edges_collection_end(graphar::EdgesCollection &collection) {
  return std::make_unique<graphar::EdgeIter>(collection.end());
}

std::unique_ptr<graphar::EdgeIter>
edges_collection_find_src(graphar::EdgesCollection &collection,
                          graphar::IdType id, const graphar::EdgeIter &from) {
  return std::make_unique<graphar::EdgeIter>(collection.find_src(id, from));
}

std::unique_ptr<graphar::EdgeIter>
edges_collection_find_dst(graphar::EdgesCollection &collection,
                          graphar::IdType id, const graphar::EdgeIter &from) {
  return std::make_unique<graphar::EdgeIter>(collection.find_dst(id, from));
}

std::shared_ptr<graphar::EdgesCollection> edges_collection_make(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &src_type, const std::string &edge_type,
    const std::string &dst_type, graphar::AdjListType adj_list_type,
    graphar::IdType vertex_chunk_begin, graphar::IdType vertex_chunk_end) {
  return ValueOrThrow(graphar::EdgesCollection::Make(
      graph_info, src_type, edge_type, dst_type, adj_list_type,
      vertex_chunk_begin, vertex_chunk_end));
}

std::shared_ptr<graphar::Expression>
expression_property(const std::string &name) {
  return graphar::_Property(name);
}

std::shared_ptr<graphar::Expression> expression_literal_bool(bool value) {
  return graphar::_Literal(value);
}

std::shared_ptr<graphar::Expression> expression_literal_i32(int32_t value) {
  return graphar::_Literal(value);
}

std::shared_ptr<graphar::Expression> expression_literal_i64(int64_t value) {
  return graphar::_Literal(value);
}

std::shared_ptr<graphar::Expression> expression_literal_f64(double value) {
  return graphar::_Literal(value);
}

std::shared_ptr<graphar::Expression>
expression_literal_string(const std::string &value) {
  return graphar::_Literal(value);
}

std::shared_ptr<graphar::Expression>
expression_equal(const std::shared_ptr<graphar::Expression> &lhs,
                 const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_Equal(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_not_equal(const std::shared_ptr<graphar::Expression> &lhs,
                     const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_NotEqual(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_greater_than(const std::shared_ptr<graphar::Expression> &lhs,
                        const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_GreaterThan(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_greater_equal(const std::shared_ptr<graphar::Expression> &lhs,
                         const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_GreaterEqual(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_less_than(const std::shared_ptr<graphar::Expression> &lhs,
                     const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_LessThan(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_less_equal(const std::shared_ptr<graphar::Expression> &lhs,
                      const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_LessEqual(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_and(const std::shared_ptr<graphar::Expression> &lhs,
               const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_And(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_or(const std::shared_ptr<graphar::Expression> &lhs,
              const std::shared_ptr<graphar::Expression> &rhs) {
  return graphar::_Or(lhs, rhs);
}

std::shared_ptr<graphar::Expression>
expression_not(const std::shared_ptr<graphar::Expression> &expr) {
  return graphar::_Not(expr);
}

} // namespace graphar_rs
