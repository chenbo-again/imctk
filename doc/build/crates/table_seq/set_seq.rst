===============
``mod set_seq``
===============


.. rust:module:: table_seq::set_seq
   :index: 0
   :vis: pub


   .. rust:use:: table_seq::set_seq
      :used_name: self


   .. rust:use:: core::fmt
      :used_name: fmt


   .. rust:use:: std::mem::replace
      :used_name: replace


   .. rust:use:: std::borrow::Borrow
      :used_name: Borrow


   .. rust:use:: std::hash::Hash
      :used_name: Hash


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: table_seq::table_seq::TableSeq
      :used_name: TableSeq


   .. rust:use:: table_seq::table_seq::SubtableIter
      :used_name: SubtableIter


   .. rubric:: Structs and Unions


   .. rust:struct:: table_seq::set_seq::SetIter
      :index: 1
      :vis: pub
      :toc: struct SetIter
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SetIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Iterator yielding references to a set's elements.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::set_seq::SetIter::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetIter","target":"SetIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Default for SetIter


      .. rust:impl:: table_seq::set_seq::SetIter::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetIter","target":"SetIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for SetIter


      .. rust:impl:: table_seq::set_seq::SetIter::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetIter","target":"SetIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for SetIter


   .. rust:struct:: table_seq::set_seq::SetSeq
      :index: 1
      :vis: pub
      :toc: struct SetSeq
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SetSeq"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Indexed sequence of hash sets.
      
      This type serves as a memory and runtime efficient replacement for `Vec<HashSet<T>>`. In
      particular, it is optimized for the use-case where the vast majority of contained sets are tiny,
      each having 16 or fewer entries, while still allowing for a small but significant fraction of
      tables to be large.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::set_seq::SetSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeq","target":"SetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeq


         .. rubric:: Functions


         .. rust:function:: table_seq::set_seq::SetSeq::at
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"at"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"set"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Provides shared access to the set at a given index, panics if out-of-bounds.
            
            This is used instead of [`std::ops::Index`], as it returns a value of the custom
            reference-like [`SetSeqSet`] type.
            
            Panics if `set >= self.len()`.

         .. rust:function:: table_seq::set_seq::SetSeq::at_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"at_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"set"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Provides mutable access to the set at a given index, panics if out-of-bounds.
            
            This is used instead of [`std::ops::IndexMut`], as it returns a value of the custom
            reference-like [`SetSeqSetMut`] type.
            
            Panics if `set >= self.len()`.

         .. rust:function:: table_seq::set_seq::SetSeq::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Discards all sets in the sequence.

         .. rust:function:: table_seq::set_seq::SetSeq::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"set"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

            Provides shared access to the set at a given index.
            
            This returns `None` if `set >= self.len()`.

         .. rust:function:: table_seq::set_seq::SetSeq::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"set"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

            Provides mutable access to the set at a given index.
            
            This returns `None` if `set >= self.len()`.

         .. rust:function:: table_seq::set_seq::SetSeq::grow_for
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"grow_for"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"set"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Ensures that the sequence contains a set at the given index by appending emtpy sets if the
            sequence was too short.
            
            Provides mutable access to the set at the given index.

         .. rust:function:: table_seq::set_seq::SetSeq::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` if the sequence of sets is empty.

         .. rust:function:: table_seq::set_seq::SetSeq::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of sets in the sequence.

         .. rust:function:: table_seq::set_seq::SetSeq::resize
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"resize"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"sets"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Resizes the sequence by appending empty sets or discarding trailing sets.

      .. rust:impl:: table_seq::set_seq::SetSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeq","target":"SetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeq


      .. rust:impl:: table_seq::set_seq::SetSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeq","target":"SetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeq


      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::set_seq::SetSeq::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetSeq","target":"SetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Default for SetSeq


      .. rust:impl:: table_seq::set_seq::SetSeq::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetSeq","target":"SetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for SetSeq


   .. rust:struct:: table_seq::set_seq::SetSeqSet
      :index: 1
      :vis: pub
      :toc: struct SetSeqSet
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Shared read-only access to a set of a [`SetSeq`].

      .. rubric:: Implementations


      .. rust:impl:: table_seq::set_seq::SetSeqSet
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeqSet


         .. rubric:: Functions


         .. rust:function:: table_seq::set_seq::SetSeqSet::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` when the set is empty.

         .. rust:function:: table_seq::set_seq::SetSeqSet::iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetIter","target":"SetIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator over the elements of the set.

         .. rust:function:: table_seq::set_seq::SetSeqSet::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of elements the set contains.

      .. rust:impl:: table_seq::set_seq::SetSeqSet
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeqSet


         .. rubric:: Functions


         .. rust:function:: table_seq::set_seq::SetSeqSet::contains
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"contains"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Checks whether a given value is an element of the set.

         .. rust:function:: table_seq::set_seq::SetSeqSet::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Returns a reference to a given value of the set.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::set_seq::SetSeqSet::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for SetSeqSet


   .. rust:struct:: table_seq::set_seq::SetSeqSetMut
      :index: 1
      :vis: pub
      :toc: struct SetSeqSetMut
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Exclusive mutable access to a set of a [`SetSeq`].

      .. rubric:: Implementations


      .. rust:impl:: table_seq::set_seq::SetSeqSetMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeqSetMut


         .. rubric:: Functions


         .. rust:function:: table_seq::set_seq::SetSeqSetMut::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Discards all elements of the set.

      .. rust:impl:: table_seq::set_seq::SetSeqSetMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"BuildHasher","target":"BuildHasher"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl SetSeqSetMut


         .. rubric:: Functions


         .. rust:function:: table_seq::set_seq::SetSeqSetMut::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Inserts a value into the set.
            
            If the value is already present, the given value is discarded and the set is not modified.
            
            Returns `true` when a new element was inserted and `false` when the value was already
            present.

         .. rust:function:: table_seq::set_seq::SetSeqSetMut::insert_unique_unchecked
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert_unique_unchecked"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"}]

            Inserts a value into the set, assuming it is not yet present.
            
            It is an error to call this with a value already present. Doing so results in unspecified
            (but still safe) behavior.

         .. rust:function:: table_seq::set_seq::SetSeqSetMut::remove
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Removes a given value from the set.
            
            Returns `true` when the value was removed and `false` if the value was not present.

         .. rust:function:: table_seq::set_seq::SetSeqSetMut::replace
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"replace"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Inserts a value into the set, replacing an existing equal value.
            
            If the value is already present, the already present value is removed from the set before
            inserting the new value.
            
            Returns the already present value that was removed or `None` if the value was not present
            before.

         .. rust:function:: table_seq::set_seq::SetSeqSetMut::take
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"take"},{"type":"punctuation","value":"<"},{"type":"name","value":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"newline"},{"type":"keyword","value":"where"},{"type":"newline"},{"type":"indent"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Borrow","target":"Borrow"},{"type":"punctuation","value":"<"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":">"},{"type":"punctuation","value":","},{"type":"newline"},{"type":"indent"},{"type":"link","value":"Q","target":"Q"},{"type":"punctuation","value":": "},{"type":"link","value":"Hash","target":"Hash"},{"type":"punctuation","value":" + "},{"type":"link","value":"Eq","target":"Eq"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"}]

            Removes and returns a given value from the set.
            
            Returns `None` when the given value was not present.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::set_seq::SetSeqSetMut::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for SetSeqSetMut


      .. rust:impl:: table_seq::set_seq::SetSeqSetMut::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for SetSeqSetMut

