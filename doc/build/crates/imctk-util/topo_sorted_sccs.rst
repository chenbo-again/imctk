========================
``mod topo_sorted_sccs``
========================


.. rust:module:: imctk_util::topo_sorted_sccs
   :index: 0
   :vis: pub


   .. rust:use:: imctk_util::topo_sorted_sccs
      :used_name: self


   .. rust:use:: imctk_util::vec_sink::VecSink
      :used_name: VecSink


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rubric:: Traits


   .. rust:trait:: imctk_util::topo_sorted_sccs::DfsIndexMap
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"DfsIndexMap"}]

      Temporary storage for the depth-first-search visit order (DFS index) used by [`TopoSortedSccs`].
      
      For graphs that use dense sequential node identifiers, this is usually the provided [`IdVec`]
      implementation.

      .. rubric:: Types


      .. rust:type:: imctk_util::topo_sorted_sccs::DfsIndexMap::DfsIndex
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"DfsIndex"}]

         Type used to represent the DFS index.
         
         This should be an [`Id`] type that is sufficiently large for enumerating the nodes visited
         during topological sorting.

      .. rust:type:: imctk_util::topo_sorted_sccs::DfsIndexMap::Node
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"type"},{"type":"space"},{"type":"name","value":"Node"}]

         Type used to identify nodes.

      .. rubric:: Functions


      .. rust:function:: imctk_util::topo_sorted_sccs::DfsIndexMap::dfs_index
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"dfs_index"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"DfsIndex"},{"type":"punctuation","value":">"}]

         Get the DFS index for a node.

      .. rust:function:: imctk_util::topo_sorted_sccs::DfsIndexMap::set_dfs_index
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"set_dfs_index"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":", "},{"type":"name","value":"dfs_index"},{"type":"punctuation","value":": "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":"::"},{"type":"name","value":"DfsIndex"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         Insert, update or delete the DFS index for node.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_util::topo_sorted_sccs::IdVec::DfsIndexMap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"Node"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"DfsIndex"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"DfsIndexMap","target":"DfsIndexMap"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Node","target":"Node"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"DfsIndex","target":"DfsIndex"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]
         :toc: impl DfsIndexMap for IdVec


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_util::topo_sorted_sccs::TopoSortedSccs
      :index: 1
      :vis: pub
      :toc: struct TopoSortedSccs
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"TopoSortedSccs"},{"type":"punctuation","value":"<"},{"type":"name","value":"D"},{"type":"punctuation","value":": "},{"type":"link","value":"DfsIndexMap","target":"DfsIndexMap"},{"type":"punctuation","value":">"}]

      Performs topological sorting and detects strongly connected components (SCCs).
      
      This implements Tarjan's linear time strongly connected components algorithm. It uses a
      non-recursive implementation to support arbitrary large graphs without the risk of causing stack
      overflows.

      .. rubric:: Implementations


      .. rust:impl:: imctk_util::topo_sorted_sccs::TopoSortedSccs
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"D"},{"type":"punctuation","value":": "},{"type":"link","value":"DfsIndexMap","target":"DfsIndexMap"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"TopoSortedSccs","target":"TopoSortedSccs"},{"type":"punctuation","value":"<"},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":">"}]
         :toc: impl TopoSortedSccs


         .. rubric:: Functions


         .. rust:function:: imctk_util::topo_sorted_sccs::TopoSortedSccs::process
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"process"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"nodes"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"incoming"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":", "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"component_callback"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"punctuation","value":")"}]

            Processes all newly discovered strongly connected components for a set of nodes and all their
            predecessors. Strongly connected components are emitted in topological order.
            
            SCCs already processed in previous calls to [`process`][Self::process] or
            [`process_one`][Self::process_one] will not be processed again. Note that any already
            processed SCC must precede any newly discovered SCC in any valid topological order.
            
            The `nodes` parameter defines the set of nodes to process (including their predecessors).
            
            The `incoming` parameter is a callback that defines the graph to operatre on. It has to
            produce the incoming neighbors for a given node by appending them to the provided
            [`VecSink`].
            
            The `component_callback` parameter is invoked, in topological order, for every newly
            discovered SCC.

         .. rust:function:: imctk_util::topo_sorted_sccs::TopoSortedSccs::process_one
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"process_one"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"incoming"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":", "},{"type":"link","value":"VecSink","target":"VecSink"},{"type":"punctuation","value":"<"},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"component_callback"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"punctuation","value":")"}]

            Processes all newly discovered components for a given node and all its predecessors.
            Strongly connected components are emitted in topological order.
            
            This is equivalent to calling [`process`][Self::process] with a nodes parameter producing
            the single given node.

         .. rust:function:: imctk_util::topo_sorted_sccs::TopoSortedSccs::processed
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"processed"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node"},{"type":"punctuation","value":": "},{"type":"link","value":"D","target":"D"},{"type":"punctuation","value":"::"},{"type":"name","value":"Node"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Checks whether a node and its predecessors have already been processed.
