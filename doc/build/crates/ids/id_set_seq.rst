==================
``mod id_set_seq``
==================


.. rust:module:: imctk_ids::id_set_seq
   :index: 0
   :vis: pub


   .. rust:use:: imctk_ids::id_set_seq
      :used_name: self


   .. rust:use:: std::marker::PhantomData
      :used_name: PhantomData


   .. rust:use:: std::hash::BuildHasherDefault
      :used_name: BuildHasherDefault


   .. rust:use:: table_seq::set_seq::SetSeqSetMut
      :used_name: SetSeqSetMut


   .. rust:use:: table_seq::set_seq::SetSeq
      :used_name: SetSeq


   .. rust:use:: table_seq::set_seq::SetSeqSet
      :used_name: SetSeqSet


   .. rust:use:: zwohash::ZwoHasher
      :used_name: ZwoHasher


   .. rust:use:: imctk_ids::Id
      :used_name: Id


   .. rubric:: Structs and Unions


   .. rust:struct:: imctk_ids::id_set_seq::IdSetSeq
      :index: 1
      :vis: pub
      :toc: struct IdSetSeq
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"IdSetSeq"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"}]

      Indexed sequence of hash sets.
      
      This type serves as a memory and runtime efficient replacement for `IdVec<I, HashSet<T>>`.

      .. rubric:: Implementations


      .. rust:impl:: imctk_ids::id_set_seq::IdSetSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IdSetSeq","target":"IdSetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl IdSetSeq


         .. rubric:: Functions


         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::at
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"at"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Provides shared access to the set for a given id, panics if the id is out-of-bounds.
            
            This is used instead of [`std::ops::Index`], as it returns a value of the custom
            reference-like [`SetSeqSet`] type.
            
            Panics if `id.id_index() >= self.len()`.

         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::at_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"at_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Provides mutable access to the set for a given id, panics if the id is out-of-bounds.
            
            This is used instead of [`std::ops::IndexMut`], as it returns a value of the custom
            reference-like [`SetSeqSetMut`] type.
            
            Panics if `id.id_index() >= self.len()`.

         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Discards all sets in the sequence.

         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"SetSeqSet","target":"SetSeqSet"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

            Provides shared access to the set for a given id.
            
            This returns `None` if `id.id_index() >= self.len()`.

         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"}]

            Provides mutable access to the set for a given id.
            
            This returns `None` if `id.id_index() >= self.len()`.

         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::grow_for
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"grow_for"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"id"},{"type":"punctuation","value":": "},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SetSeqSetMut","target":"SetSeqSetMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]

            Ensures that the sequence contains a set for the given id by appending emtpy sets if the
            sequence was too short.
            
            Provides mutable access to the set for the given id.

         .. rust:function:: imctk_ids::id_set_seq::IdSetSeq::resize
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"resize"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"len"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Resizes the sequence by appending empty sets or discarding trailing sets.

      .. rubric:: Traits implemented


      .. rust:impl:: imctk_ids::id_set_seq::IdSetSeq::Deref
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"ops"},{"type":"punctuation","value":"::"},{"type":"name","value":"Deref"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSetSeq","target":"IdSetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Deref for IdSetSeq


      .. rust:impl:: imctk_ids::id_set_seq::IdSetSeq::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"I"},{"type":"punctuation","value":": "},{"type":"link","value":"Id","target":"Id"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"S"},{"type":"punctuation","value":": "},{"type":"link","value":"Default","target":"Default"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"IdSetSeq","target":"IdSetSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"I","target":"I"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"S","target":"S"},{"type":"punctuation","value":">"}]
         :toc: impl Default for IdSetSeq

