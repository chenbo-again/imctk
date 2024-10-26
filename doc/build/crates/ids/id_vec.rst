==============
``mod id_vec``
==============


.. rust:module:: imctk_ids::id_vec
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ids::id_vec
      :used_name: self


   .. rust:use:: core::ops::IndexMut
      :used_name: IndexMut


   .. rust:use:: core::marker::PhantomData
      :used_name: PhantomData


   .. rust:use:: core::ops::DerefMut
      :used_name: DerefMut


   .. rust:use:: core::ops::Deref
      :used_name: Deref


   .. rust:use:: core::ops::Index
      :used_name: Index


   .. rust:use:: core::iter::FusedIterator
      :used_name: FusedIterator


   .. rust:use:: core::fmt::Debug
      :used_name: Debug


   .. rust:use:: core::fmt
      :used_name: fmt


   .. rust:use:: imctk_ids::IdRange
      :used_name: IdRange


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ids::id_vec::IdSlice
      :index: 1
      :vis: pub
      :toc: struct IdSlice
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      Transparent [`[V]`][`slice`] wrapper, representing a collection that maps `K` keys to `V`
      values.
      
      It has entries `(k, v)` with `v` being the item at position [`k.id_index()`][Id::id_index] of
      the wrapped slice. This means the keys always span a contiguous range of ids starting at at
      [`K::MIN_ID`][Id::MIN_ID], having index `0`.
      
      It comes with a guarantee that all entries have distinct and valid keys. Depending on the used
      id type, this guarantee limits the maximum allowed length of the wrapped slice.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::id_vec::IdSlice
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl IdSlice


         .. rubric:: Functions


         .. rust:function:: imctk_ids::id_vec::IdSlice::from_mut_slice
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_mut_slice"},{"type":"punctuation","value":"("},{"type":"name","value":"slice"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates a mutable `IdSlice` reference from a mutable slice of values.
            
            # Panics
            
            Panics when `K` cannot index the full length of the slice.

         .. rust:function:: imctk_ids::id_vec::IdSlice::from_mut_slice_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_mut_slice_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"slice"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates a mutable `IdSlice` reference from a mutable slice of values without bounds
            checking.
            
            # Safety
            
            The caller has to ensure that `K` can index the full length of the slice.

         .. rust:function:: imctk_ids::id_vec::IdSlice::from_slice
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_slice"},{"type":"punctuation","value":"("},{"type":"name","value":"slice"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"}]

            Creates an `IdSlice` reference from a slice of values.
            
            # Panics
            
            Panics when `K` cannot index the full length of the slice.

         .. rust:function:: imctk_ids::id_vec::IdSlice::from_slice_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_slice_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"slice"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":"]"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"}]

            Creates an `IdSlice` reference from a slice of values without bounds checking.
            
            # Safety
            
            The caller has to ensure that `K` can index the full length of the slice.

         .. rust:function:: imctk_ids::id_vec::IdSlice::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns a reference to the value associated with the given key.
            
            Returns `None` when the key is out-of-bounds.

         .. rust:function:: imctk_ids::id_vec::IdSlice::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns a mutable reference to the value associated with the given key.
            
            Returns `None` when the key is out-of-bounds.

         .. rust:function:: imctk_ids::id_vec::IdSlice::get_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_unchecked"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"}]

            Returns a reference to the value associated with the given key without bounds checking.
            
            # Safety
            
            The caller has to ensure that the key is valid for this mapping.

         .. rust:function:: imctk_ids::id_vec::IdSlice::get_unchecked_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_unchecked_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Returns a mutable reference to the value associated with the given key without bounds
            checking.
            
            # Safety
            
            The caller has to ensure that the key is valid for this mapping.

         .. rust:function:: imctk_ids::id_vec::IdSlice::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` if there are no entries in the collection.

         .. rust:function:: imctk_ids::id_vec::IdSlice::iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"::"},{"type":"name","value":"IntoIter"}]

            Returns an iterator over all entries using value references.
            
            Each entry is a `(K, &V)` pair.

         .. rust:function:: imctk_ids::id_vec::IdSlice::iter_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"::"},{"type":"name","value":"IntoIter"}]

            Returns an iterator over all entries using mutable value references.
            
            Each entry is a `(K, &mut V)` pair.

         .. rust:function:: imctk_ids::id_vec::IdSlice::keys
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"keys"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IdRange","target":"IdRange"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"}]

            Returns the keys as a contiguous range of ids.

         .. rust:function:: imctk_ids::id_vec::IdSlice::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of entries in the collection.

         .. rust:function:: imctk_ids::id_vec::IdSlice::swap
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"swap"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key_a"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"key_b"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"}]

            Swaps the values associated with the two given keys.

         .. rust:function:: imctk_ids::id_vec::IdSlice::values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"punctuation","value":"["},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":"]"}]

            Returns the values as a slice.
            
            This also provides access to the wrapped slice.

         .. rust:function:: imctk_ids::id_vec::IdSlice::values_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"values_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"punctuation","value":"["},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":"]"}]

            Returns the values as a mutable slice.
            
            This also provides mutable access to the wrapped slice.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::id_vec::IdSlice::Index
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Index","target":"Index"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Index for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::IndexMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IndexMut","target":"IndexMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl IndexMut for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::Index
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Index","target":"Index"},{"type":"punctuation","value":"<"},{"type":"link","value":"IdRange","target":"IdRange"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Index for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::IndexMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IndexMut","target":"IndexMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"IdRange","target":"IdRange"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl IndexMut for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::PartialEq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"U"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"PartialEq","target":"PartialEq"},{"type":"punctuation","value":"<"},{"type":"link","value":"U","target":"U"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"PartialEq","target":"PartialEq"},{"type":"punctuation","value":"<"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"U","target":"U"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl PartialEq for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::Eq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Eq","target":"Eq"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Eq for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::PartialOrd
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"PartialOrd","target":"PartialOrd"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"PartialOrd","target":"PartialOrd"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl PartialOrd for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::Ord
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Ord","target":"Ord"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Ord","target":"Ord"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Ord for IdSlice


      .. rust:impl:: imctk_ids::id_vec::IdSlice::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSlice","target":"IdSlice"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for IdSlice


   .. rust:struct:: imctk_ids::id_vec::IdVec
      :index: 1
      :vis: pub
      :toc: struct IdVec
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IdVec"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      Transparent [`Vec`] wrapper, representing a collection that maps `K` keys to `V` values.
      
      It has entries `(k, v)` with `v` being the item at position [`k.id_index()`][Id::id_index] of
      the wrapped vector. This means the keys always span a contiguous range of ids starting at at
      [`K::MIN_ID`][Id::MIN_ID], having index `0`.
      
      It comes with a guarantee that all entries have distinct and valid keys. Depending on the used
      id type, this guarantee limits the maximum allowed length of the wrapped vector.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::id_vec::IdVec
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl IdVec


         .. rubric:: Functions


         .. rust:function:: imctk_ids::id_vec::IdVec::append
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"append"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"entries"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Inserts a vector of values using the smallest available ids as keys.

         .. rust:function:: imctk_ids::id_vec::IdVec::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Removes all entries of the collection.

         .. rust:function:: imctk_ids::id_vec::IdVec::drain_all_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"drain_all_values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"vec"},{"type":"punctuation","value":"::"},{"type":"name","value":"Drain"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Removes all entries, returning an iterator yielding the removed entries in order.

         .. rust:function:: imctk_ids::id_vec::IdVec::extend_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"extend_values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"iter"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Inserts values from an iterator using the smallest available ids as keys.

         .. rust:function:: imctk_ids::id_vec::IdVec::from_vec
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_vec"},{"type":"punctuation","value":"("},{"type":"name","value":"vec"},{"type":"punctuation","value":": "},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates an `IdVec` from a vector of values.

         .. rust:function:: imctk_ids::id_vec::IdVec::from_vec_mut_ref
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_vec_mut_ref"},{"type":"punctuation","value":"("},{"type":"name","value":"vec"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates a mutable `IdVec` reference from a mutable reference to a vector of values.
            
            # Panics
            
            Panics when `K` cannot index the full length of the vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::from_vec_mut_ref_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_vec_mut_ref_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"vec"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates a mutable `IdVec` reference from a mutable reference to a vector of values without
            bounds checking.
            
            # Safety
            
            The caller has to ensure that `K` can index the full length of the vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::from_vec_ref
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_vec_ref"},{"type":"punctuation","value":"("},{"type":"name","value":"vec"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"}]

            Creates an `IdVec` reference from a reference to a vector of values.
            
            # Panics
            
            Panics when `K` cannot index the full length of the vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::from_vec_ref_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_vec_ref_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"vec"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Self","target":"Self"}]

            Creates an `IdVec` reference from a reference to a vector of values without bounds checking.
            
            # Safety
            
            The caller has to ensure that `K` can index the full length of the vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::from_vec_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"from_vec_unchecked"},{"type":"punctuation","value":"("},{"type":"name","value":"vec"},{"type":"punctuation","value":": "},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Creates an `IdVec` from a vector of values without bounds checking.
            
            # Safety
            
            The caller has to ensure that `K` can index the full length of the vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::grow_for_key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"grow_for_key"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"}]

            Appends default values until there is an entry with the given key.

         .. rust:function:: imctk_ids::id_vec::IdVec::grow_for_key_with
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"grow_for_key_with"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Appends values using the given closure until there is an entry with the given key.

         .. rust:function:: imctk_ids::id_vec::IdVec::into_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_values"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Converts this collection into a vector of the contained values.
            
            This also provides owned access to the wrapped vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::modify_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"modify_values"},{"type":"punctuation","value":"<"},{"type":"name","value":"R"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"R","target":"R"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"R","target":"R"}]

            Modifies the underlying [`Vec`] of values.

         .. rust:function:: imctk_ids::id_vec::IdVec::next_unused_key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"next_unused_key"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"K","target":"K"}]

            Returns the id with the smallest available index.
            
            This is the same key that would be used when calling [`push`][Self::push].

         .. rust:function:: imctk_ids::id_vec::IdVec::pop
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"pop"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Removes and returns the entry with the id having the largest used index.

         .. rust:function:: imctk_ids::id_vec::IdVec::push
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"push"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"}]

            Inserts a value as a new entry, using the id with the smallest available index as key.
            
            This returns the used key and a mutable reference to the just inserted value.

         .. rust:function:: imctk_ids::id_vec::IdVec::resize
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"resize"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"len"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Clone","target":"Clone"}]

            Resizes the collection, creating new entries by cloning the given value.

         .. rust:function:: imctk_ids::id_vec::IdVec::resize_with
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"resize_with"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"len"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"}]

            Resizes the collection, creating new entries by calling the given closure.

         .. rust:function:: imctk_ids::id_vec::IdVec::retain_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"retain_values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

            Retains only those entries where the given predicate holds for the value. Keys following the
            first removed entry are changed such that entry position and key id index remain the same
            for all entries.

         .. rust:function:: imctk_ids::id_vec::IdVec::swap_remove
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"swap_remove"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Removes the entry with the given key, returning its value. If there is an entry with a
            larger key id, its key is changed to the key of the removed entry.

         .. rust:function:: imctk_ids::id_vec::IdVec::truncate
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"truncate"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"len"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Shrinks the collection, dropping all but the first `len` entries.

         .. rust:function:: imctk_ids::id_vec::IdVec::values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns a reference to the values.
            
            This also provides immutable access to the wrapped vector.

         .. rust:function:: imctk_ids::id_vec::IdVec::values_mut_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"values_mut_unchecked"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Vec","target":"Vec"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns a mutable reference to the values.
            
            This also provides mutable access to the wrapped vector.
            
            # Safety
            
            The caller has to ensure that the returned vector does not grow in excess of what `K` can
            index.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::id_vec::IdVec::Clone
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Clone","target":"Clone"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Clone","target":"Clone"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Clone for IdVec


      .. rust:impl:: imctk_ids::id_vec::IdVec::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Default for IdVec


      .. rust:impl:: imctk_ids::id_vec::IdVec::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Deref","target":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for IdVec


      .. rust:impl:: imctk_ids::id_vec::IdVec::DerefMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"DerefMut","target":"DerefMut"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl DerefMut for IdVec


      .. rust:impl:: imctk_ids::id_vec::IdVec::IntoIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl IntoIterator for IdVec


      .. rust:impl:: imctk_ids::id_vec::IdVec::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for IdVec


      .. rust:impl:: imctk_ids::id_vec::IdVec::From
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"From","target":"From"},{"type":"punctuation","value":"<"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl From for IdVec

