======================
``mod indexed_id_vec``
======================


.. rust:module:: imctk_ids::indexed_id_vec
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ids::indexed_id_vec
      :used_name: self


   .. rust:use:: std::hash::BuildHasherDefault
      :used_name: BuildHasherDefault


   .. rust:use:: std::borrow::Borrow
      :used_name: Borrow


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: std::mem::replace
      :used_name: replace


   .. rust:use:: std::hash::Hash
      :used_name: Hash


   .. rust:use:: std::ops::Deref
      :used_name: Deref


   .. rust:use:: hashbrown::HashTable
      :used_name: HashTable


   .. rust:use:: zwohash::ZwoHasher
      :used_name: ZwoHasher


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rust:use:: imctk_ids::id_vec::IdVec
      :used_name: IdVec


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ids::indexed_id_vec::IndexedIdVec
      :index: 1
      :vis: pub
      :toc: struct IndexedIdVec
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      A collection type that extends an [`IdVec`] with a hash set index of the contained values.
      
      This imlements [`Deref<Target=IdVec<K, V>>`][`Deref`] so that all read-only [`IdVec`] methods
      work as-is.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::indexed_id_vec::IndexedIdVec
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl IndexedIdVec


         .. rubric:: Functions


         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Removes all entries from the collection.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::contains_value
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"contains_value"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Returns whether there already is an entry for the given value.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::drain
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"drain"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"vec"},{"type":"punctuation","value":"::"},{"type":"name","value":"Drain"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns an iterator over all entries and removes them from the collection.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::extend_values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"extend_values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"iter"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Inserts values from an iterator using the smallset available ids as keys.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::get_key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_key"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Retrieves the key for a value in the set.
            
            Returns `None` if the set doesn't contain the given value.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"}]

            Tries to insert a new value using the next available key.
            
            If the set didn't already contain the new value, it returns the newly allocated key, a
            reference to the inserted value and `true`. If the value was already in the set, it returns
            the key and value reference for the existing entry and `false`.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::into_id_vec
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_id_vec"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"IdVec","target":"IdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns the underlying [`IdVec`], discarding the set index.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::pop
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"pop"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"}]

            Removes and returns the last entry.
            
            Returns `None` if the collection is empty.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::replace
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"replace"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Result","target":"Result"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":">"}]

            Replaces the value at a given key with a new value.
            
            If the set didn't already contain the new value, it returns the previously value at the
            given key wrapped in `Ok`. Otherwise it returns the key that already contains the given
            value wrapped in `Err`.

         .. rust:function:: imctk_ids::indexed_id_vec::IndexedIdVec::swap_remove_value
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"swap_remove_value"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Removes a value and closes any resulting gap among the keys by changing the key of the last
            entry to be the newly unused key.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::indexed_id_vec::IndexedIdVec::FromIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"},{"type":"punctuation","value":" + "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"FromIterator","target":"FromIterator"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl FromIterator for IndexedIdVec


      .. rust:impl:: imctk_ids::indexed_id_vec::IndexedIdVec::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Deref","target":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for IndexedIdVec


      .. rust:impl:: imctk_ids::indexed_id_vec::IndexedIdVec::Clone
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Clone","target":"Clone"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"Clone","target":"Clone"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Clone","target":"Clone"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Clone for IndexedIdVec


      .. rust:impl:: imctk_ids::indexed_id_vec::IndexedIdVec::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for IndexedIdVec


      .. rust:impl:: imctk_ids::indexed_id_vec::IndexedIdVec::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IndexedIdVec","target":"IndexedIdVec"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Default for IndexedIdVec

