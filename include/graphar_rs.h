#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "graphar/expression.h"
#include "graphar/fwd.h"
#include "graphar/graph_info.h"
#include "graphar/high-level/edges_builder.h"
#include "graphar/high-level/graph_reader.h"
#include "graphar/high-level/vertices_builder.h"
#include "graphar/types.h"
#include "graphar/version_parser.h"
#include "rust/cxx.h"

// https://github.com/dtolnay/cxx/issues/741
// https://github.com/dtolnay/cxx/issues/774
// Used for `create_graph_info`
using SharedVertexInfo = std::shared_ptr<graphar::VertexInfo>;
using SharedEdgeInfo = std::shared_ptr<graphar::EdgeInfo>;
using SharedPropertyGroup = std::shared_ptr<graphar::PropertyGroup>;

namespace graphar {
using ConstInfoVersion = const InfoVersion;
}

namespace graphar_rs {
using i32 = int32_t;
using i64 = int64_t;
using f32 = float;
using f64 = double;

// `DataType`
rust::String to_type_name(const graphar::DataType &type);

// InfoVersion
std::shared_ptr<graphar::InfoVersion> new_info_version(int version);
std::shared_ptr<const graphar::InfoVersion> new_const_info_version(int version);

// GraphInfo
std::shared_ptr<graphar::GraphInfo> load_graph_info(const std::string &path);

std::shared_ptr<graphar::GraphInfo> create_graph_info(
    const std::string &name, const graphar::VertexInfoVector &vertex_infos,
    const graphar::EdgeInfoVector &edge_infos,
    const rust::Vec<rust::String> &labels, const std::string &prefix,
    std::shared_ptr<const graphar::InfoVersion> version);
void graph_info_save(const graphar::GraphInfo &graph_info,
                     const std::string &path);
std::unique_ptr<std::string>
graph_info_dump(const graphar::GraphInfo &graph_info);

std::unique_ptr<graphar::Property>
new_property(const std::string &name,
             const std::shared_ptr<graphar::DataType> &type = nullptr,
             bool is_primary = false, bool is_nullable = true,
             graphar::Cardinality cardinality = graphar::Cardinality::SINGLE);
const std::string &property_get_name(const graphar::Property &prop);
const std::shared_ptr<graphar::DataType> &
property_get_type(const graphar::Property &prop);
bool property_is_primary(const graphar::Property &prop);
bool property_is_nullable(const graphar::Property &prop);
graphar::Cardinality property_get_cardinality(const graphar::Property &prop);
std::unique_ptr<graphar::Property>
property_clone(const graphar::Property &prop);

std::unique_ptr<std::vector<graphar::Property>> new_properties();
void push_property(std::vector<graphar::Property> &properties,
                   std::unique_ptr<graphar::Property> prop);

std::unique_ptr<graphar::PropertyGroupVector> new_property_group_vec();
void push_property_group(
    graphar::PropertyGroupVector &vec,
    std::shared_ptr<graphar::PropertyGroup> property_group);

// VertexInfo
void vertex_info_save(const graphar::VertexInfo &v, const std::string &path);
std::unique_ptr<std::string> vertex_info_dump(const graphar::VertexInfo &v);
std::shared_ptr<graphar::VertexInfo>
create_vertex_info(const rust::String &type, graphar::IdType chunk_size,
                   const graphar::PropertyGroupVector &property_group,
                   const rust::Vec<rust::String> &labels,
                   const rust::String &prefix,
                   std::shared_ptr<const graphar::InfoVersion> version);

// builder::Vertex
std::unique_ptr<graphar::builder::Vertex> new_vertex();
// Vertex add property
void vertex_add_property_bool(graphar::builder::Vertex &v,
                              const std::string &name, bool val);
void vertex_add_property_i32(graphar::builder::Vertex &v,
                             const std::string &name, int32_t val);
void vertex_add_property_i64(graphar::builder::Vertex &v,
                             const std::string &name, int64_t val);
void vertex_add_property_f32(graphar::builder::Vertex &v,
                             const std::string &name, float val);
void vertex_add_property_f64(graphar::builder::Vertex &v,
                             const std::string &name, double val);
void vertex_add_property_string(graphar::builder::Vertex &v,
                                const std::string &name,
                                const std::string &val);

// builder::VerticesBuilder
std::shared_ptr<graphar::builder::VerticesBuilder>
new_vertices_builder(const std::shared_ptr<graphar::VertexInfo> &vertex_info,
                     const std::string &path_prefix,
                     graphar::IdType start_index);
void add_vertex(graphar::builder::VerticesBuilder &builder,
                graphar::builder::Vertex &v);
void vertices_dump(graphar::builder::VerticesBuilder &builder);

// AdjacentListVector
std::unique_ptr<graphar::AdjacentListVector> new_adjacent_list_vec();
void push_adjacent_list(graphar::AdjacentListVector &v,
                        std::shared_ptr<graphar::AdjacentList> adj_list);

// EdgeInfo
void edge_info_save(const graphar::EdgeInfo &edge_info,
                    const std::string &path);
std::unique_ptr<std::string> edge_info_dump(const graphar::EdgeInfo &edge_info);

// builder::Edge
std::unique_ptr<graphar::builder::Edge> new_edge(graphar::IdType src_id,
                                                 graphar::IdType dst_id);
void edge_add_property_bool(graphar::builder::Edge &e, const std::string &name,
                            bool val);
void edge_add_property_i32(graphar::builder::Edge &v, const std::string &name,
                           int32_t val);
void edge_add_property_i64(graphar::builder::Edge &v, const std::string &name,
                           int64_t val);
void edge_add_property_f32(graphar::builder::Edge &v, const std::string &name,
                           float val);
void edge_add_property_f64(graphar::builder::Edge &v, const std::string &name,
                           double val);
void edge_add_property_string(graphar::builder::Edge &v,
                              const std::string &name, const std::string &val);

// builder::EdgeBuilder
std::shared_ptr<graphar::builder::EdgesBuilder>
new_edges_builder(const std::shared_ptr<graphar::EdgeInfo> &edge_info,
                  const std::string &path_prefix,
                  graphar::AdjListType adj_list_type,
                  graphar::IdType vertices_num);
void add_edge(graphar::builder::EdgesBuilder &builder,
              graphar::builder::Edge &v);
void edges_dump(graphar::builder::EdgesBuilder &builder);

// Vertex

#define DECL_VERTEX_PROPERTY_FUNC(type)                                        \
  type vertex_property_##type(const graphar::Vertex &vertex,                   \
                              const std::string &name);

DECL_VERTEX_PROPERTY_FUNC(bool)
DECL_VERTEX_PROPERTY_FUNC(i32)
DECL_VERTEX_PROPERTY_FUNC(i64)
DECL_VERTEX_PROPERTY_FUNC(f32)
DECL_VERTEX_PROPERTY_FUNC(f64)
rust::String vertex_property_string(const graphar::Vertex &vertex,
                                    const std::string &name);

// Edge
#define DECL_EDGE_PROPERTY_FUNC(type)                                          \
  type edge_property_##type(const graphar::Edge &edge, const std::string &name);

DECL_EDGE_PROPERTY_FUNC(bool)
DECL_EDGE_PROPERTY_FUNC(i32)
DECL_EDGE_PROPERTY_FUNC(i64)
DECL_EDGE_PROPERTY_FUNC(f32)
DECL_EDGE_PROPERTY_FUNC(f64)
rust::String edge_property_string(const graphar::Edge &edge,
                                  const std::string &name);

// VertexIter
std::unique_ptr<graphar::Vertex> vertex_iter_deref(graphar::VertexIter &iter);
bool vertex_iter_eq(const std::unique_ptr<graphar::VertexIter> &lhs,
                    const std::unique_ptr<graphar::VertexIter> &rhs);
graphar::IdType vertex_iter_id(graphar::VertexIter &iter);
bool vertex_iter_property_bool(graphar::VertexIter &iter,
                               const std::string &name);
int32_t vertex_iter_property_i32(graphar::VertexIter &iter,
                                 const std::string &name);
int64_t vertex_iter_property_i64(graphar::VertexIter &iter,
                                 const std::string &name);
float vertex_iter_property_f32(graphar::VertexIter &iter,
                               const std::string &name);
double vertex_iter_property_f64(graphar::VertexIter &iter,
                                const std::string &name);
rust::String vertex_iter_property_string(graphar::VertexIter &iter,
                                         const std::string &name);
bool vertex_iter_has_label(graphar::VertexIter &iter, const std::string &label);
std::unique_ptr<std::vector<std::string>>
vertex_iter_labels(graphar::VertexIter &iter);
void vertex_iter_next(graphar::VertexIter &iter);

// EdgeIter
std::unique_ptr<graphar::Edge> edge_iter_deref(graphar::EdgeIter &iter);
bool edge_iter_eq(const std::unique_ptr<graphar::EdgeIter> &lhs,
                  const std::unique_ptr<graphar::EdgeIter> &rhs);
graphar::IdType edge_iter_source(graphar::EdgeIter &iter);
graphar::IdType edge_iter_destination(graphar::EdgeIter &iter);
bool edge_iter_property_bool(graphar::EdgeIter &iter, const std::string &name);
int32_t edge_iter_property_i32(graphar::EdgeIter &iter,
                               const std::string &name);
int64_t edge_iter_property_i64(graphar::EdgeIter &iter,
                               const std::string &name);
float edge_iter_property_f32(graphar::EdgeIter &iter, const std::string &name);
double edge_iter_property_f64(graphar::EdgeIter &iter, const std::string &name);
rust::String edge_iter_property_string(graphar::EdgeIter &iter,
                                       const std::string &name);
void edge_iter_to_begin(graphar::EdgeIter &iter);
void edge_iter_next(graphar::EdgeIter &iter);
bool edge_iter_next_src(graphar::EdgeIter &iter);
bool edge_iter_next_dst(graphar::EdgeIter &iter);
bool edge_iter_next_src_with_id(graphar::EdgeIter &iter, graphar::IdType id);
bool edge_iter_next_dst_with_id(graphar::EdgeIter &iter, graphar::IdType id);

// VerticesCollection
std::shared_ptr<graphar::VerticesCollection>
vertices_collection_make(const std::shared_ptr<graphar::GraphInfo> &graph_info,
                         const std::string &type);
std::unique_ptr<graphar::VertexIter>
vertices_collection_begin(graphar::VerticesCollection &vc);
std::unique_ptr<graphar::VertexIter>
vertices_collection_end(graphar::VerticesCollection &vc);
std::unique_ptr<graphar::VertexIter>
vertices_collection_find(graphar::VerticesCollection &vc, graphar::IdType id);
std::unique_ptr<std::vector<graphar::IdType>>
filter_by_label_with_chunk(graphar::VerticesCollection &vc,
                           const std::vector<std::string> &filter_labels,
                           std::vector<graphar::IdType> &new_vaild_chunk);
std::unique_ptr<std::vector<graphar::IdType>>
filter_by_label(graphar::VerticesCollection &vc,
                const std::vector<std::string> &filter_labels);
std::unique_ptr<std::vector<graphar::IdType>>
filter_by_acero(const graphar::VerticesCollection &vc,
                const std::vector<std::string> &filter_labels);

std::unique_ptr<std::vector<graphar::IdType>>
filter_by_property_name_with_chunk(
    graphar::VerticesCollection &vc, const std::string &property_name,
    std::shared_ptr<graphar::Expression> filter_expr,
    std::vector<graphar::IdType> &new_vaild_chunk);
std::unique_ptr<std::vector<graphar::IdType>>
filter_by_property_name(graphar::VerticesCollection &vc,
                        const std::string &property_name,
                        std::shared_ptr<graphar::Expression> filter_expr);

std::shared_ptr<graphar::VerticesCollection> vertices_collection_with_label(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &type, const std::string &label);
std::shared_ptr<graphar::VerticesCollection> vertices_collection_with_labels(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &type, const std::vector<std::string> &labels);
std::shared_ptr<graphar::VerticesCollection> vertices_collection_with_property(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &type, const std::string &property_name,
    const std::shared_ptr<graphar::Expression> &filter);

// EdgesCollection
std::unique_ptr<graphar::EdgeIter>
edges_collection_begin(graphar::EdgesCollection &collection);
std::unique_ptr<graphar::EdgeIter>
edges_collection_end(graphar::EdgesCollection &collection);
std::unique_ptr<graphar::EdgeIter>
edges_collection_find_src(graphar::EdgesCollection &collection,
                          graphar::IdType id, const graphar::EdgeIter &from);
std::unique_ptr<graphar::EdgeIter>
edges_collection_find_dst(graphar::EdgesCollection &collection,
                          graphar::IdType id, const graphar::EdgeIter &from);
std::shared_ptr<graphar::EdgesCollection> edges_collection_make(
    const std::shared_ptr<graphar::GraphInfo> &graph_info,
    const std::string &src_type, const std::string &edge_type,
    const std::string &dst_type, graphar::AdjListType adj_list_type,
    graphar::IdType vertex_chunk_begin, graphar::IdType vertex_chunk_end);

// Expression helpers
std::shared_ptr<graphar::Expression>
expression_property(const std::string &name);
std::shared_ptr<graphar::Expression>
expression_property_by_property(const graphar::Property &property);
std::shared_ptr<graphar::Expression> expression_literal_bool(bool value);
std::shared_ptr<graphar::Expression> expression_literal_i32(int32_t value);
std::shared_ptr<graphar::Expression> expression_literal_i64(int64_t value);
std::shared_ptr<graphar::Expression> expression_literal_f32(float value);
std::shared_ptr<graphar::Expression> expression_literal_f64(double value);
std::shared_ptr<graphar::Expression>
expression_literal_string(const std::string &value);
std::shared_ptr<graphar::Expression>
expression_equal(const std::shared_ptr<graphar::Expression> &lhs,
                 const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_not_equal(const std::shared_ptr<graphar::Expression> &lhs,
                     const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_greater_than(const std::shared_ptr<graphar::Expression> &lhs,
                        const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_greater_equal(const std::shared_ptr<graphar::Expression> &lhs,
                         const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_less_than(const std::shared_ptr<graphar::Expression> &lhs,
                     const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_less_equal(const std::shared_ptr<graphar::Expression> &lhs,
                      const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_and(const std::shared_ptr<graphar::Expression> &lhs,
               const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_or(const std::shared_ptr<graphar::Expression> &lhs,
              const std::shared_ptr<graphar::Expression> &rhs);
std::shared_ptr<graphar::Expression>
expression_not(const std::shared_ptr<graphar::Expression> &expr);

} // namespace graphar_rs
