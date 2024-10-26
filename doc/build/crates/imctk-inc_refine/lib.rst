==========================
Crate ``imctk_inc_refine``
==========================


.. rust:crate:: imctk_inc_refine
   :index: 0

   Incremental partition refinement.
   
   This provides an implementation of an incremental partition refinement data structure.
   Incrementality is allowed in two different ways:
   
   1. It is always possible to keep refining the partition, splitting classes into multiple
      classes. This is a usual requirement for partition refinement use cases.
   2. It is possible to add new elements of an unknown class to the partition. This means that the
      data structure doesn't represent a partition that happens to be incrementally updated, but
      represents what we will call an incremental partition.
   
   Such an incremental partition can be viewed as a forest of directed trees with the incremental
   partition elements as vertices. Two elements are considered to be candidates for sharing a class
   when either is an ancestor of the other.
   
   Any (sub-)tree can be refined using a class invariant function by labeling the subtree using the
   class invariant values, duplicating the (sub-)tree for every observed value (all rooted at the
   same parent) and then for each copy contracting all vertices that do not have the value
   corresponding to that specific copy.
   
   When implementing this, the contracted copies can be constructed directly, avoiding any
   intermediate blowup.
   
   The roots of the trees can serve as class representatives of the finest non-incremental
   partition that is a coarsening of the incremental partition.
   
   A new element of an unknown class can be added by turning the forest into a single tree with the
   new element as root.
   
   The data structure is implemented by maintaining:
   
   * A linked list representing an euler tour of the forest viewed as a tree with a virtual root
     not corresponding to an element, associating the unique incoming edge with each element. (An
     alternative perspective is that the forest is represented as an s-expr and we maintain a
     linked list of opening and closing parens.)
   
     This requires fixing an order between siblings, which can be done arbitrarily.
   
   * An order maintenance data structure to quickly determine the relative order of two linked list
     elements. In this setting, this allows us to check whether one element is the parent of
     another.
   
   * Lazily updated shortcut links to find the first sibling among the children of the same parent
     and to find the root of any element.
   
   * Monotone linked list item timestamps that are updated whenever we move such an item into a new
     contracted "copy" and stored along the shortcut links so they can be invalidated when a target
     node moves. Note that the largest subclass during refinement doesn't have to move as we will
     implicitly obtain the contracted tree after having moved out all other elements.

   .. rust:use:: imctk_inc_refine
      :used_name: self


   .. rust:use:: std::marker::PhantomData
      :used_name: PhantomData


   .. rust:use:: std::mem::swap
      :used_name: swap


   .. rust:use:: std::mem::take
      :used_name: take


   .. rust:use:: std::hash::Hash
      :used_name: Hash


   .. rust:use:: imctk_ids::Id32
      :used_name: Id32


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: zwohash::HashMap
      :used_name: HashMap


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_inc_refine::IncrementalRefinement
      :index: 1
      :vis: pub
      :toc: struct IncrementalRefinement
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IncrementalRefinement"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"}]

      Incremental partition refinement data structure.

      .. rubric:: Implementations


      .. rust:impl:: imctk_inc_refine::IncrementalRefinement
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IncrementalRefinement","target":"IncrementalRefinement"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl IncrementalRefinement


         .. rubric:: Functions


         .. rust:function:: imctk_inc_refine::IncrementalRefinement::ancestral_sibling_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"ancestral_sibling_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of items that are siblings, ancestors or siblings of an ancestor of the
            given item.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::child_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"child_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Counts the number of items directly below the given item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::child_iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"child_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns an item over the direct children of a given item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::contains_item
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"contains_item"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether a given item is tracked as part of the incremental partition.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::descendant_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"descendant_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Counts the number of items below the given item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::equiv
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"equiv"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"items"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"["},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":"; "},{"type":"literal","value":"2"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"}]

            Make two items equivalent.
            
            This will remove the ancestor item and insert it directly above the descendant item.
            
            Panics when either given item is not currently tracked or when neither of the given item is
            an ancestor of the other item.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::first_sibling
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"first_sibling"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Returns the representative item among the siblings of a given item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::insert_item
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert_item"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Starts tracking a given item as part of the incremental partition.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::is_ancestor_of
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_ancestor_of"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"outer"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"inner"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether `outer` is an ancestor of `inner`.
            
            Panics when either given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::is_isolated
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_isolated"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether the given item is isolated, i.e. both a root and leaf item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::is_leaf
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_leaf"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether the given item is a leaf item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::is_root
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_root"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether the given item is a root item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::item_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"item_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of currently tracked items.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::nonisolated_iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"nonisolated_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns an iterator over all non-isolated items.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::nonisolated_with_root_iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"nonisolated_with_root_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"punctuation","value":"("},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns an iterator over all non-isolated items together with their corresponding root item.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::nonleaf_root_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"nonleaf_root_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Counts the number of root items that are not leaf items.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::nonleaf_root_iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"nonleaf_root_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns an iterator over the root items that are not leaf items.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::parent
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"parent"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns the parent item of the given item.
            
            Returns `None` when the class containing the given item is a root class. Panics when the
            given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::postorder_descendants_iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"postorder_descendants_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":" + "},{"type":"lifetime","value":"'_"}]

            Returns an iterator that traverses the descendants of the given item in post-order.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::refine_all
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"refine_all"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"keys"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"RefinementKeys","target":"RefinementKeys"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"}]

            Refines all items using the partition defined by the return value of the `key` closure.
            
            Users need to ensure that the `key` closure is deterministic. If this condition is violated
            the behavior is memory-safe but otherwise undefined.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::refine_items
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"refine_items"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"items"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Refines the incremental partition by making the listed items distinct from any non-listed
            items.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::refine_subtree
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"refine_subtree"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"keys"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"RefinementKeys","target":"RefinementKeys"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"name","value":"subtree"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"}]

            Refines all items contained strictly below a given item using the partition defined by the
            return value of the `key` closure.
            
            The class of items not contained strictly below the given item is considered unknown, i.e.
            the given item and any of its ancestors will still be considered potentially equal to the
            items strictly below the given item.
            
            Users need to ensure that the `key` closure is deterministic. If this condition is violated
            the behavior is memory-safe but otherwise undefined.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::remove_item
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove_item"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Stops tracking a given item as part of the incremental partition.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::root
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"root"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Returns the representative item for the root class containing the given item.
            
            Panics when the given item is not currently tracked.

         .. rust:function:: imctk_inc_refine::IncrementalRefinement::root_count
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"root_count"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of root items in the incremental partition.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_inc_refine::IncrementalRefinement::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IncrementalRefinement","target":"IncrementalRefinement"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for IncrementalRefinement


   .. rust:struct:: imctk_inc_refine::RefinementKeys
      :index: 1
      :vis: pub
      :toc: struct RefinementKeys
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"RefinementKeys"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":">"}]

      Temporary storage needed for refinement.
      
      The [`Default`] trait implementation is the only public API provided by this type.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_inc_refine::RefinementKeys::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"RefinementKeys","target":"RefinementKeys"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"}]
         :toc: impl Default for RefinementKeys

