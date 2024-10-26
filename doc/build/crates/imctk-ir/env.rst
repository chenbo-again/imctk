===========
``mod env``
===========


.. rust:module:: imctk_ir::env
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ir::env
      :used_name: self


   .. rust:use:: std::mem::swap
      :used_name: swap


   .. rust:use:: std::mem::take
      :used_name: take


   .. rust:use:: imctk_ids::Id32
      :used_name: Id32


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rust:use:: imctk_transparent::NewtypeCast
      :used_name: NewtypeCast


   .. rust:use:: imctk_transparent::SubtypeCast
      :used_name: SubtypeCast


   .. rust:use:: imctk_util::give_take::Take
      :used_name: Take


   .. rust:use:: imctk_ir::index::UsesIndex
      :used_name: UsesIndex


   .. rust:use:: imctk_ir::index::DefsIndex
      :used_name: DefsIndex


   .. rust:use:: super::var::Var
      :used_name: Var


   .. rust:use:: super::node::generic::TermNode
      :used_name: TermNode


   .. rust:use:: super::node::builder::NodeBuilder
      :used_name: NodeBuilder


   .. rust:use:: super::node::generic::DynTerm
      :used_name: DynTerm


   .. rust:use:: super::node::generic::dyn_term_into_dyn_term_node
      :used_name: dyn_term_into_dyn_term_node


   .. rust:use:: super::index::StructuralHashIndex
      :used_name: StructuralHashIndex


   .. rust:use:: super::node::NodeId
      :used_name: NodeId


   .. rust:use:: super::node::generic::Node
      :used_name: Node


   .. rust:use:: super::node::generic::Term
      :used_name: Term


   .. rust:use:: super::node::collections::nodes::Nodes
      :used_name: Nodes


   .. rust:use:: super::var::Lit
      :used_name: Lit


   .. rust:use:: super::node::generic::DynNode
      :used_name: DynNode


   .. rust:use:: super::node::collections::buf::NodeBuf
      :used_name: NodeBuf


   .. rust:use:: super::node::builder::NodeBuilderDyn
      :used_name: NodeBuilderDyn


   .. rust:use:: super::index::DynamicIndex
      :used_name: DynamicIndex


   .. rust:use:: imctk_ir::env::LitMultimap
      :used_name: LitMultimap


   .. rust:use:: imctk_ir::env::EnvUpdates
      :used_name: EnvUpdates


   .. rust:use:: imctk_ir::env::VarMultimap
      :used_name: VarMultimap


   .. rubric:: Traits


   .. rust:trait:: imctk_ir::env::EnvWrapper
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"trait"},{"type":"space"},{"type":"name","value":"EnvWrapper"}]

      Types that wrap and expose different aspects of an [environment][`Env`].

      .. rubric:: Functions


      .. rust:function:: imctk_ir::env::EnvWrapper::env
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"env"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"}]

         Returns a reference to the wrapped environment.

      .. rust:function:: imctk_ir::env::EnvWrapper::env_mut
         :index: 2
         :vis: pub
         :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"env_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Env","target":"Env"}]

         Returns a mutable reference to the wrapped environment.

      .. rubric:: Implemented for


      .. rust:impl:: imctk_ir::env::RawEnvNodes::EnvWrapper
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"EnvWrapper","target":"EnvWrapper"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"RawEnvNodes","target":"RawEnvNodes"}]
         :toc: impl EnvWrapper for RawEnvNodes


      .. rust:impl:: imctk_ir::env::Env::EnvWrapper
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"EnvWrapper","target":"EnvWrapper"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"Env","target":"Env"}]
         :toc: impl EnvWrapper for Env


      .. rust:impl:: imctk_ir::env::DefBuilder::EnvWrapper
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"EnvWrapper","target":"EnvWrapper"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"DefBuilder","target":"DefBuilder"}]
         :toc: impl EnvWrapper for DefBuilder


   .. rubric:: Enums


   .. rust:enum:: imctk_ir::env::NodeRole
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"enum"},{"type":"space"},{"type":"name","value":"NodeRole"}]

      Indicates whether a node is the primary definition or an equivalent definition of a variable or
      alternatively whether it is a constraint on the node inputs.

      .. rust:struct:: imctk_ir::env::NodeRole::PrimaryDef
         :index: 2
         :vis: pub
         :toc: PrimaryDef
         :layout: [{"type":"name","value":"PrimaryDef"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"}]

         Used for a node that is the primary definition of a variable.

      .. rust:struct:: imctk_ir::env::NodeRole::Equivalence
         :index: 2
         :vis: pub
         :toc: Equivalence
         :layout: [{"type":"name","value":"Equivalence"},{"type":"punctuation","value":"("},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"}]

         Used for a node that gives an equivalent definition of a variable.

      .. rust:struct:: imctk_ir::env::NodeRole::Constraint
         :index: 2
         :vis: pub
         :toc: Constraint
         :layout: [{"type":"name","value":"Constraint"}]

         Used for a node that constrains its inputs.

   .. rust:enum:: imctk_ir::env::VarDef
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"enum"},{"type":"space"},{"type":"name","value":"VarDef"}]

      Definition of a variable, either the defining node or an equivalent literal.

      .. rust:struct:: imctk_ir::env::VarDef::Node
         :index: 2
         :vis: pub
         :toc: Node
         :layout: [{"type":"name","value":"Node"},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":")"}]

         The referenced node is the primary definition of the variable.

      .. rust:struct:: imctk_ir::env::VarDef::Equiv
         :index: 2
         :vis: pub
         :toc: Equiv
         :layout: [{"type":"name","value":"Equiv"},{"type":"punctuation","value":"("},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"}]

         The referenced literal is the primary definition of the variable.

   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ir::env::Env
      :index: 1
      :vis: pub
      :toc: struct Env
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"Env"}]

      An environment for storing and maintaining the internal representation.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::env::Env
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Env","target":"Env"}]
         :toc: impl Env


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::Env::def_node
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"def_node"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":">"}]

            Returns the node that is the primary definition of a variable.

         .. rust:function:: imctk_ir::env::Env::def_node_with_id
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"def_node_with_id"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"link","value":"DynNode","target":"DynNode"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Returns the node id and node for the primary definition of a variable.

         .. rust:function:: imctk_ir::env::Env::duplicate_node_with_var_map
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"duplicate_node_with_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":", "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"}]

            Applies a variable map to a copy of node and adds it into the same environment.
            
            Note that this will also use the variable map for any output of the node, see
            [`Self::duplicate_term_with_var_map`] for a version that will use a fresh or an existing
            already equivalent variable instead.

         .. rust:function:: imctk_ir::env::Env::duplicate_term_with_var_map
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"duplicate_term_with_var_map"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"node_id"},{"type":"punctuation","value":": "},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"var_map"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"},{"type":"punctuation","value":", "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Applies a variable map to a copy of a node's term and adds it into the same environment,
            returning the fresh or existing already equivalent output variable.
            
            This will panic if the given node is not a [`TermNode`], see
            [`Self::duplicate_node_with_var_map`] for a version that supports all nodes and also remaps
            the output variable.

         .. rust:function:: imctk_ir::env::Env::lit_repr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"lit_repr"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"lit"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Returns the canonical representative literal equivalent to the given literal.
            
            This will perform path compression on the internal union-find data structure used to keep
            track of equivalent literals. To look up a canonical representative with a read-only
            environment reference (and thus without performing path-compression) [`VarDefs::lit_repr`]
            can be used via [`var_defs`][`Self::var_defs`].

      .. rust:impl:: imctk_ir::env::Env
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Env","target":"Env"}]
         :toc: impl Env


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::Env::lookup_term
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"lookup_term"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Term","target":"Term"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"term"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Finds an existing node that assigns the given term to a variable.

   .. rust:struct:: imctk_ir::env::EnvUpdates
      :index: 1
      :vis: pub
      :toc: struct EnvUpdates
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"EnvUpdates"}]

      
      

      .. rust:variable:: imctk_ir::env::EnvUpdates::equivs
         :index: 2
         :vis: pub
         :toc: equivs
         :layout: [{"type":"name","value":"equivs"},{"type":"punctuation","value":": "},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"}]


      .. rust:variable:: imctk_ir::env::EnvUpdates::steady
         :index: 2
         :vis: pub
         :toc: steady
         :layout: [{"type":"name","value":"steady"},{"type":"punctuation","value":": "},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"}]


      .. rust:variable:: imctk_ir::env::EnvUpdates::nodes
         :index: 2
         :vis: pub
         :toc: nodes
         :layout: [{"type":"name","value":"nodes"},{"type":"punctuation","value":": "},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"NodeId","target":"NodeId"},{"type":"punctuation","value":">"}]


   .. rust:struct:: imctk_ir::env::LitMultimap
      :index: 1
      :vis: pub
      :toc: struct LitMultimap
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"LitMultimap"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Equivalence aware multimap with [`Lit`] keys.
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::env::multimap::LitMultimap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"LitMultimap","target":"LitMultimap"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl LitMultimap


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::multimap::LitMultimap::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Removes all entries from the collection.

      .. rust:impl:: imctk_ir::env::multimap::LitMultimap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"BitXorAssign","target":"BitXorAssign"},{"type":"punctuation","value":"<"},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"LitMultimap","target":"LitMultimap"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl LitMultimap


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::multimap::LitMultimap::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"lit"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Inserts a value for a given literal key.
            
            Returns `false` when the value was already present for the given literal.

         .. rust:function:: imctk_ir::env::multimap::LitMultimap::insert_repr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert_repr"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"lit"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Inserts a value for a given literal key, assuming the literal is the representative for its
            equivalence class.
            
            Returns `false` when the value was already present for the given literal.

         .. rust:function:: imctk_ir::env::multimap::LitMultimap::merge_equivs
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"merge_equivs"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":")"}]

            Merges newly equivalent literals.

      .. rust:impl:: imctk_ir::env::multimap::LitMultimap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"LitMultimap","target":"LitMultimap"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl LitMultimap


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::multimap::LitMultimap::lit_entries
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"lit_entries"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"lit"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"BitXor","target":"BitXor"},{"type":"punctuation","value":"<"},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"::"},{"type":"name","value":"Output"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'a"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"BitXor","target":"BitXor"},{"type":"punctuation","value":"<"},{"type":"link","value":"Pol","target":"Pol"},{"type":"punctuation","value":">"}]

            Returns an iterator over the values associated with a given literal.

         .. rust:function:: imctk_ir::env::multimap::LitMultimap::var_entries
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"var_entries"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator over the values associated with a given variable.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::env::multimap::LitMultimap::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"LitMultimap","target":"LitMultimap"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Default for LitMultimap


   .. rust:struct:: imctk_ir::env::VarDefs
      :index: 1
      :vis: pub
      :toc: struct VarDefs
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"VarDefs"}]

      Maintains the primary definitions of all variables.
      
      This combines a polarity-aware union-find data structure over all equivalent literals with a map
      storing the defining node for each equivalence class representative.
      
      It also maintains a best-effort upper bound on the level for each variable. The level is defined
      as the height of the DAG that defines a variable in terms of the inputs. While these bounds are
      correctly updated when adding term nodes, equivalences and during egraph rebuilding, other
      operations may not do so. This means correctness of these bounds is not a general environment
      invariant.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::env::VarDefs
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"VarDefs","target":"VarDefs"}]
         :toc: impl VarDefs


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::VarDefs::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether no variables have been assigned.

         .. rust:function:: imctk_ir::env::VarDefs::is_steady
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_steady"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether the variable is known to have a steady value.
            
            A steady value remains constant at the value it has in the initial state.

         .. rust:function:: imctk_ir::env::VarDefs::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of assigned variables.

         .. rust:function:: imctk_ir::env::VarDefs::level_bound
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"level_bound"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u32","target":"u32"}]

            Returns a best-effort upper bound on the level of the node that defines a given variable.

         .. rust:function:: imctk_ir::env::VarDefs::lit_repr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"lit_repr"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"lit"},{"type":"punctuation","value":": "},{"type":"link","value":"Lit","target":"Lit"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Lit","target":"Lit"}]

            Returns the canonical representative literal equivalent to the given literal.

         .. rust:function:: imctk_ir::env::VarDefs::repr_vars
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"repr_vars"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"DoubleEndedIterator","target":"DoubleEndedIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns an iterator over all representative variables.
            
            Representative variables are variables not known to be equivalent to other variables or
            their negation.
            
            This will yield a double ended iterator of representative variables ordered by increasing
            id.

         .. rust:function:: imctk_ir::env::VarDefs::var_def
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"var_def"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"VarDef","target":"VarDef"},{"type":"punctuation","value":">"}]

            Retrieves the primary definition of a variable.
            
            Returns `None` when the environment does not contain a designed primary definition.

         .. rust:function:: imctk_ir::env::VarDefs::var_repr
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"var_repr"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Var","target":"Var"}]

            Returns the canonical representative variable for the given variable.
            
            This ignores polarities, and thus might return a variable that's equivalent to the negation
            of the passed variable. Most of the time, [`lit_repr`][Self::lit_repr] should be used
            instead.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ir::env::VarDefs::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"VarDefs","target":"VarDefs"}]
         :toc: impl Default for VarDefs


   .. rust:struct:: imctk_ir::env::VarMultimap
      :index: 1
      :vis: pub
      :toc: struct VarMultimap
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"VarMultimap"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Equivalence aware multimap with [`Var`] keys.
      
      

      .. rubric:: Implementations


      .. rust:impl:: imctk_ir::env::multimap::VarMultimap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"VarMultimap","target":"VarMultimap"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl VarMultimap


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::multimap::VarMultimap::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Inserts a value for a given variable key.
            
            Returns `false` when the value was already present for the given variable.

         .. rust:function:: imctk_ir::env::multimap::VarMultimap::merge_equivs
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"merge_equivs"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"env"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Env","target":"Env"},{"type":"punctuation","value":")"}]

            Merges newly equivalent (modulo negation) variables.

      .. rust:impl:: imctk_ir::env::multimap::VarMultimap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"VarMultimap","target":"VarMultimap"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl VarMultimap


         .. rubric:: Functions


         .. rust:function:: imctk_ir::env::multimap::VarMultimap::var_entries
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"var_entries"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"var"},{"type":"punctuation","value":": "},{"type":"link","value":"Var","target":"Var"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator over the values associated with a given variable.
