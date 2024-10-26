===============
``mod map_seq``
===============


.. rust:module:: table_seq::map_seq
   :index: 0
   :vis: pub


   .. rust:use:: table_seq::map_seq
      :used_name: self


   .. rust:use:: table_seq::table_seq::TableSeq
      :used_name: TableSeq


   .. rust:use:: table_seq::table_seq::SubtableIterMut
      :used_name: SubtableIterMut


   .. rust:use:: table_seq::table_seq::SubtableIter
      :used_name: SubtableIter


   .. rust:use:: table_seq::table_seq
      :used_name: table_seq


   .. rust:use:: core::fmt
      :used_name: fmt


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: std::mem
      :used_name: mem


   .. rust:use:: std::borrow::Borrow
      :used_name: Borrow


   .. rust:use:: std::hash::Hash
      :used_name: Hash


   .. rubric:: Enums


   .. rust:enum:: table_seq::map_seq::Entry
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"enum"},{"type":"space"},{"type":"name","value":"Entry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      A view into a single entry in a map, which may either be vacant or occupied.
      
      This `enum` is constructed from the [`entry`] method on [`MapSeqMapMut`].

      .. rust:struct:: table_seq::map_seq::Entry::Vacant
         :index: 2
         :vis: pub
         :toc: Vacant
         :layout: [{"type":"name","value":"Vacant"},{"type":"punctuation","value":"("},{"type":"link","value":"VacantEntry","target":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         A vacant entry.

      .. rust:struct:: table_seq::map_seq::Entry::Occupied
         :index: 2
         :vis: pub
         :toc: Occupied
         :layout: [{"type":"name","value":"Occupied"},{"type":"punctuation","value":"("},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         An occupied entry.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::map_seq::Entry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Entry","target":"Entry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Entry


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::Entry::and_modify
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"and_modify"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Provides in-place mutable access to an occupied entry before any potential inserts into the map.

         .. rust:function:: table_seq::map_seq::Entry::insert_entry
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert_entry"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Sets the value of the entry, and returns an `OccupiedEntry`.

         .. rust:function:: table_seq::map_seq::Entry::or_insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or_insert"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"default"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.

         .. rust:function:: table_seq::map_seq::Entry::or_insert_with
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or_insert_with"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"default"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Ensures a value is in the entry by inserting the result of the default function if empty,
            and returns a mutable reference to the value in the entry.

         .. rust:function:: table_seq::map_seq::Entry::or_insert_with_key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or_insert_with_key"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Ensures a value is in the entry by inserting, if empty, the result of the default function.
            This method allows for generating key-derived values for insertion by providing the default
            function a reference to the key that was moved during the `.entry(key)` method call.
            
            The reference to the moved key is provided so that cloning or copying the key is
            unnecessary, unlike with `.or_insert_with(|| ... )`.

      .. rust:impl:: table_seq::map_seq::Entry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Entry","target":"Entry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Entry


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::Entry::or_default
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or_default"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Ensures a value is in the entry by inserting the default value if empty,
            and returns a mutable reference to the value in the entry.

   .. rubric:: Structs and Unions


   .. rust:struct:: table_seq::map_seq::MapIter
      :index: 1
      :vis: pub
      :toc: struct MapIter
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      Iterator yielding references to a map's keys and values.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapIter::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapIter","target":"MapIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Default for MapIter


      .. rust:impl:: table_seq::map_seq::MapIter::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapIter","target":"MapIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for MapIter


      .. rust:impl:: table_seq::map_seq::MapIter::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapIter","target":"MapIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for MapIter


   .. rust:struct:: table_seq::map_seq::MapIterMut
      :index: 1
      :vis: pub
      :toc: struct MapIterMut
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      Iterator yielding references to a map's keys and values, with mutable value references.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapIterMut::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapIterMut","target":"MapIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Default for MapIterMut


      .. rust:impl:: table_seq::map_seq::MapIterMut::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapIterMut","target":"MapIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for MapIterMut


      .. rust:impl:: table_seq::map_seq::MapIterMut::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapIterMut","target":"MapIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for MapIterMut


   .. rust:struct:: table_seq::map_seq::MapKeys
      :index: 1
      :vis: pub
      :toc: struct MapKeys
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapKeys"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      Iterator yielding references to a map's keys.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapKeys::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapKeys","target":"MapKeys"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Default for MapKeys


      .. rust:impl:: table_seq::map_seq::MapKeys::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapKeys","target":"MapKeys"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for MapKeys


      .. rust:impl:: table_seq::map_seq::MapKeys::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapKeys","target":"MapKeys"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for MapKeys


   .. rust:struct:: table_seq::map_seq::MapSeq
      :index: 1
      :vis: pub
      :toc: struct MapSeq
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapSeq"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Indexed sequence of hash maps.
      
      This type serves as a memory and runtime efficient replacement for `Vec<HashMap<K, V>>`. In
      particular, it is optimized for the use-case where the vast majority of contained maps are tiny,
      each having 16 or fewer entries, while still allowing for a small but significant fraction of
      maps to be large.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::map_seq::MapSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeq","target":"MapSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeq


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::MapSeq::at
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"at"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"map"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Provides shared access to the map at a given index, panics if out-of-bounds.
            
            This is used instead of [`std::ops::Index`], as it returns a value of the custom
            reference-like [`MapSeqMap`] type.
            
            Panics if `map >= self.len()`.

         .. rust:function:: table_seq::map_seq::MapSeq::at_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"at_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"map"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Provides mutable access to the map at a given index, panics if out-of-bounds.
            
            This is used instead of [`std::ops::IndexMut`], as it returns a value of the custom
            reference-like [`MapSeqMapMut`] type.
            
            Panics if `map >= self.len()`.

         .. rust:function:: table_seq::map_seq::MapSeq::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Discards all maps in the sequence.

         .. rust:function:: table_seq::map_seq::MapSeq::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"map"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

            Provides shared access to the map at a given index.
            
            This returns `None` if `map >= self.len()`.

         .. rust:function:: table_seq::map_seq::MapSeq::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"map"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

            Provides mutable access to the map at a given index.
            
            This returns `None` if `map >= self.len()`.

         .. rust:function:: table_seq::map_seq::MapSeq::grow_for
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"grow_for"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"map"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Ensures that the sequence contains a map at the given index by appending emtpy maps if the
            sequence was too short.
            
            Provides mutable access to the map at the given index.

         .. rust:function:: table_seq::map_seq::MapSeq::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` if the sequence of maps is empty.

         .. rust:function:: table_seq::map_seq::MapSeq::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of maps in the sequence.

         .. rust:function:: table_seq::map_seq::MapSeq::resize
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"resize"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"maps"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Resizes the sequence by appending empty maps or discarding trailing maps.

      .. rust:impl:: table_seq::map_seq::MapSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeq","target":"MapSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeq


      .. rust:impl:: table_seq::map_seq::MapSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeq","target":"MapSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeq


      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapSeq::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeq","target":"MapSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Default for MapSeq


      .. rust:impl:: table_seq::map_seq::MapSeq::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeq","target":"MapSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for MapSeq


   .. rust:struct:: table_seq::map_seq::MapSeqMap
      :index: 1
      :vis: pub
      :toc: struct MapSeqMap
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Shared read-only access to a map of a [`MapSeq`].

      .. rubric:: Implementations


      .. rust:impl:: table_seq::map_seq::MapSeqMap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeqMap


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::MapSeqMap::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` when the map is empty.

         .. rust:function:: table_seq::map_seq::MapSeqMap::iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapIter","target":"MapIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns an iterator over the key-value pairs of the map.

         .. rust:function:: table_seq::map_seq::MapSeqMap::keys
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"keys"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapKeys","target":"MapKeys"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns an iterator over the keys of the map.

         .. rust:function:: table_seq::map_seq::MapSeqMap::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of elements the map contains.

         .. rust:function:: table_seq::map_seq::MapSeqMap::values
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"values"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapValues","target":"MapValues"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns an iterator over the values of the map.

      .. rust:impl:: table_seq::map_seq::MapSeqMap
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeqMap


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::MapSeqMap::contains_key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"contains_key"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Returns `true` if the map contains an element for the given key.

         .. rust:function:: table_seq::map_seq::MapSeqMap::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Returns a reference to the value corresponding to the given key.

         .. rust:function:: table_seq::map_seq::MapSeqMap::get_key_value
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_key_value"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Returns a reference to the key-value pair corresponding to the given key.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapSeqMap::Clone
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Clone","target":"Clone"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Clone for MapSeqMap


      .. rust:impl:: table_seq::map_seq::MapSeqMap::Copy
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Copy","target":"Copy"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Copy for MapSeqMap


      .. rust:impl:: table_seq::map_seq::MapSeqMap::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for MapSeqMap


      .. rust:impl:: table_seq::map_seq::MapSeqMap::IntoIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl IntoIterator for MapSeqMap


      .. rust:impl:: table_seq::map_seq::MapSeqMap::Index
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"Q"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Index"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMap","target":"MapSeqMap"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"}]
         :toc: impl Index for MapSeqMap


   .. rust:struct:: table_seq::map_seq::MapSeqMapMut
      :index: 1
      :vis: pub
      :toc: struct MapSeqMapMut
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Exclusive mutable access to a map of a [`MapSeq`].

      .. rubric:: Implementations


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeqMapMut


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::MapSeqMapMut::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Discards all elements of the map.

         .. rust:function:: table_seq::map_seq::MapSeqMapMut::iter_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapIterMut","target":"MapIterMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Returns an iterator over the elements of the map, with mutable references to values.

         .. rust:function:: table_seq::map_seq::MapSeqMapMut::reborrow
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"reborrow"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Reborrow the mutable reference to the map, creating a new mutable reference with a potentially shorter lifetime.

      .. rust:impl:: table_seq::map_seq::MapSeqMapMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl MapSeqMapMut


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::MapSeqMapMut::entry
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"entry"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Entry","target":"Entry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Gets the given key's corresponding entry in the map for in-place manipulation.

         .. rust:function:: table_seq::map_seq::MapSeqMapMut::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Returns a mutable reference to the value corresponding to the given key.

         .. rust:function:: table_seq::map_seq::MapSeqMapMut::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Inserts a key-value pair into the map.
            
            If the map did not have this key present, [`None`] is returned.
            
            If the map did have this key present, the value is updated, and the old
            value is returned. The key is not updated, though; this matters for
            types that can be `==` without being identical.

         .. rust:function:: table_seq::map_seq::MapSeqMapMut::remove
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Removes a key from the map, returning the value at the key if the key was previously in the map.

         .. rust:function:: table_seq::map_seq::MapSeqMapMut::remove_entry
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove_entry"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"key"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Removes a key from the map, returning the stored key and value if the key was previously in the map.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for MapSeqMapMut


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for MapSeqMapMut


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut::IntoIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl IntoIterator for MapSeqMapMut


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut::Extend
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Extend","target":"Extend"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"}]
         :toc: impl Extend for MapSeqMapMut


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut::Extend
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'b"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Extend","target":"Extend"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'b"},{"type":"space"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'b"},{"type":"space"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Copy","target":"Copy"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":": "},{"type":"link","value":"Copy","target":"Copy"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"}]
         :toc: impl Extend for MapSeqMapMut


      .. rust:impl:: table_seq::map_seq::MapSeqMapMut::Index
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"Q"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Index"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapSeqMapMut","target":"MapSeqMapMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"}]
         :toc: impl Index for MapSeqMapMut


   .. rust:struct:: table_seq::map_seq::MapValues
      :index: 1
      :vis: pub
      :toc: struct MapValues
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"MapValues"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"}]

      Iterator yielding references to a map's values.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::map_seq::MapValues::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapValues","target":"MapValues"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Default for MapValues


      .. rust:impl:: table_seq::map_seq::MapValues::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapValues","target":"MapValues"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for MapValues


      .. rust:impl:: table_seq::map_seq::MapValues::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"MapValues","target":"MapValues"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for MapValues


   .. rust:struct:: table_seq::map_seq::OccupiedEntry
      :index: 1
      :vis: pub
      :toc: struct OccupiedEntry
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"link","value":"table_seq","target":"table_seq"},{"type":"punctuation","value":"::"},{"type":"name","value":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"MapEntry","target":"MapEntry"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

      A view into an occupied entry in a [`MapSeq`].
      It is part of the [`Entry`] enum.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::map_seq::OccupiedEntry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl OccupiedEntry


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::OccupiedEntry::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"V","target":"V"}]

            Gets a reference to the value in the entry.

         .. rust:function:: table_seq::map_seq::OccupiedEntry::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Gets a mutable reference to the value in the entry.

         .. rust:function:: table_seq::map_seq::OccupiedEntry::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Sets the value of the entry, and returns the entry’s old value.

         .. rust:function:: table_seq::map_seq::OccupiedEntry::into_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_mut"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Converts the `OccupiedEntry` into a mutable reference to the value in the entry
            with a lifetime bound to the map itself.

         .. rust:function:: table_seq::map_seq::OccupiedEntry::key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"key"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"K","target":"K"}]

            Gets a reference to the key in the entry.

         .. rust:function:: table_seq::map_seq::OccupiedEntry::remove
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Takes the value out of the entry, and returns it.

         .. rust:function:: table_seq::map_seq::OccupiedEntry::remove_entry
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove_entry"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"}]

            Take the ownership of the key and value from the map.

   .. rust:struct:: table_seq::map_seq::VacantEntry
      :index: 1
      :vis: pub
      :toc: struct VacantEntry
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"link","value":"table_seq","target":"table_seq"},{"type":"punctuation","value":"::"},{"type":"name","value":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"MapEntry","target":"MapEntry"},{"type":"punctuation","value":"<"},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":")"}]

      A view into a vacant entry in a [`MapSeq`].
      It is part of the [`Entry`] enum.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::map_seq::VacantEntry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"K"},{"type":"punctuation","value":", "},{"type":"name","value":"V"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"VacantEntry","target":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]
         :toc: impl VacantEntry


         .. rubric:: Functions


         .. rust:function:: table_seq::map_seq::VacantEntry::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"V","target":"V"}]

            Sets the value of the entry with the `VacantEntry`'s key,
            and returns a mutable reference to it.

         .. rust:function:: table_seq::map_seq::VacantEntry::insert_entry
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert_entry"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"K","target":"K"},{"type":"punctuation","value":", "},{"type":"link","value":"V","target":"V"},{"type":"punctuation","value":">"}]

            Sets the value of the entry with the `VacantEntry`'s key,
            and returns an `OccupiedEntry`.

         .. rust:function:: table_seq::map_seq::VacantEntry::into_key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_key"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"K","target":"K"}]

            Take ownership of the key.

         .. rust:function:: table_seq::map_seq::VacantEntry::key
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"key"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"K","target":"K"}]

            Gets a reference to the key that would be used when inserting a value
            through the `VacantEntry`.
