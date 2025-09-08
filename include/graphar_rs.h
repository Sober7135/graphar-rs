#include <memory>
#include <vector>

#include "graphar/fwd.h"
#include "graphar/graph_info.h"
#include "graphar/high-level/edges_builder.h"
#include "graphar/high-level/vertices_builder.h"
#include "graphar/version_parser.h"
#include "rust/cxx.h"

namespace graphar {
using ConstInfoVersion = const InfoVersion;
}

namespace graphar_rs {
std::shared_ptr<graphar::InfoVersion> new_info_version(int version);
std::shared_ptr<const graphar::InfoVersion> new_const_info_version(int version);

std::shared_ptr<graphar::GraphInfo> load_graph_info(const std::string &path);

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

void vertex_info_save(const graphar::VertexInfo &v, const std::string &path);
std::unique_ptr<std::string> vertex_info_dump(const graphar::VertexInfo &v);
std::shared_ptr<graphar::VertexInfo>
create_vertex_info(const rust::String &type, graphar::IdType chunk_size,
                   const graphar::PropertyGroupVector &property_group,
                   const rust::Vec<rust::String> &labels,
                   const rust::String &prefix,
                   std::shared_ptr<const graphar::InfoVersion> version);

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

std::shared_ptr<graphar::builder::VerticesBuilder>
new_vertices_builder(const std::shared_ptr<graphar::VertexInfo> &vertex_info,
                     const std::string &path_prefix,
                     graphar::IdType start_index);

void add_vertex(graphar::builder::VerticesBuilder &builder,
                graphar::builder::Vertex &v);

void vertices_dump(graphar::builder::VerticesBuilder &builder);

std::unique_ptr<graphar::AdjacentListVector> new_adjacent_list_vec();
void push_adjacent_list(graphar::AdjacentListVector &v,
                        std::shared_ptr<graphar::AdjacentList> adj_list);

void edge_info_save(const graphar::EdgeInfo &edge_info,
                    const std::string &path);
std::unique_ptr<std::string> edge_info_dump(const graphar::EdgeInfo &edge_info);

std::unique_ptr<graphar::builder::Edge> new_edge(graphar::IdType src_id,
                                                 graphar::IdType dst_id);
// edge add property
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

std::shared_ptr<graphar::builder::EdgesBuilder>
new_edges_builder(const std::shared_ptr<graphar::EdgeInfo> &edge_info,
                  const std::string &path_prefix,
                  graphar::AdjListType adj_list_type,
                  graphar::IdType vertices_num);

void add_edge(graphar::builder::EdgesBuilder &builder,
              graphar::builder::Edge &v);

void edges_dump(graphar::builder::EdgesBuilder &builder);

} // namespace graphar_rs
