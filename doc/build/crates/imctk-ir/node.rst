============
``mod node``
============


.. rust:module:: imctk_ir::node
   :index: 0
   :vis: pub


   .. rubric:: Modules
   .. toctree::
      :maxdepth: 1

      node/collections
      node/builder
      node/generic
      node/fine


   .. rust:use:: imctk_ir::node
      :used_name: self


   .. rust:use:: std::fmt
      :used_name: fmt


   .. rust:use:: imctk_ids::Id32
      :used_name: Id32


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::node::NodeId
      :index: 1
      :vis: pub
      :toc: struct NodeId
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"NodeId"},{"type":"punctuation","value":"("},{"type":"link","value":"Id32","target":"Id32"},{"type":"punctuation","value":")"}]

      Identifies an individual node within a collection of nodes.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::node::NodeId::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"NodeId","target":"NodeId"}]
         :toc: impl Debug for NodeId

