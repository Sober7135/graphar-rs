#include <memory>
#include <vector>

#include "graphar/expression.h"
#include "graphar/fwd.h"
#include "graphar/graph_info.h"
#include "graphar/high-level/edges_builder.h"
#include "graphar/high-level/graph_reader.h"
#include "graphar/high-level/vertices_builder.h"
#include "graphar/version_parser.h"
#include "rust/cxx.h"

namespace graphar {
using ConstInfoVersion = const InfoVersion;
}

namespace graphar_rs {
// InfoVersion
std::shared_ptr<graphar::InfoVersion> new_info_version(int version);
std::shared_ptr<const graphar::InfoVersion> new_const_info_version(int version);

std::shared_ptr<graphar::GraphInfo> load_graph_info(const std::string &path);

// GraphInfo
void graph_info_save(const graphar::GraphInfo &graph_info,
                     const std::string &path);
std::unique_ptr<std::string>
graph_info_dump(const graphar::GraphInfo &graph_info);

std::unique_ptr<std::vector<graphar::Property>> new_properties();
void push_property(std::vector<graphar::Property> &properties,
                   const std::string &name,
                   const std::shared_ptr<graphar::DataType> &type,
                   bool is_primary, bool is_nullable,
                   graphar::Cardinality cardinality);

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

// VertexIter
graphar::IdType vertex_iter_id(graphar::VertexIter &iter);
void vertex_iter_next(graphar::VertexIter &iter);
bool vertex_iter_has_label(graphar::VertexIter &iter, const std::string &label);
rust::Vec<rust::String> vertex_iter_labels(graphar::VertexIter &iter);
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

// EdgeIter
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

} // namespace graphar_rs
