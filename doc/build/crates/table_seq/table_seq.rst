=================
``mod table_seq``
=================


.. rust:module:: table_seq::table_seq
   :index: 0
   :vis: pub


   .. rust:use:: table_seq::table_seq
      :used_name: self


   .. rust:use:: core::fmt
      :used_name: fmt


   .. rust:use:: std::mem::ManuallyDrop
      :used_name: ManuallyDrop


   .. rust:use:: std::marker::PhantomData
      :used_name: PhantomData


   .. rust:use:: std::mem::MaybeUninit
      :used_name: MaybeUninit


   .. rust:use:: hashbrown::HashTable
      :used_name: HashTable


   .. rust:use:: table_seq::node_allocator::AllocatorClass
      :used_name: AllocatorClass


   .. rust:use:: table_seq::table_seq::chunk::EntryType
      :used_name: EntryType


   .. rust:use:: table_seq::table_seq::chunk::CHUNK_SIZE
      :used_name: CHUNK_SIZE


   .. rust:use:: table_seq::table_seq::chunk::CHUNK_SHIFT
      :used_name: CHUNK_SHIFT


   .. rust:use:: table_seq::table_seq::chunk::CHUNK_MASK
      :used_name: CHUNK_MASK


   .. rust:use:: table_seq::table_seq::chunk::Chunk
      :used_name: Chunk


   .. rust:use:: table_seq::table_seq::node::NodeRef
      :used_name: NodeRef


   .. rust:use:: table_seq::table_seq::node::NodeAllocator
      :used_name: NodeAllocator


   .. rust:use:: table_seq::table_seq::node::SizeClass
      :used_name: SizeClass


   .. rust:use:: table_seq::table_seq::owned::OwnedSubtableSmall
      :used_name: OwnedSubtableSmall


   .. rust:use:: table_seq::table_seq::table::SmallSubtableEntry
      :used_name: SmallSubtableEntry


   .. rust:use:: table_seq::table_seq::table::SmallSubtableOccupiedEntry
      :used_name: SmallSubtableOccupiedEntry


   .. rust:use:: table_seq::table_seq::table::SmallSubtableVacantEntry
      :used_name: SmallSubtableVacantEntry


   .. rust:use:: table_seq::table_seq::table::Subtable
      :used_name: Subtable


   .. rust:use:: table_seq::table_seq::table::SmallSubtable
      :used_name: SmallSubtable


   .. rust:use:: table_seq::table_seq::SubtableIter
      :used_name: SubtableIter


   .. rust:use:: table_seq::table_seq::OwnedSubtable
      :used_name: OwnedSubtable


   .. rust:use:: table_seq::table_seq::SubtableIterMut
      :used_name: SubtableIterMut


   .. rust:use:: table_seq::table_seq::Entry
      :used_name: Entry


   .. rust:use:: table_seq::table_seq::OccupiedEntry
      :used_name: OccupiedEntry


   .. rust:use:: table_seq::table_seq::VacantEntry
      :used_name: VacantEntry


   .. rubric:: Enums


   .. rust:enum:: table_seq::table_seq::Entry
      :index: 1
      :vis: pub
      :layout: [{"type":"keyword","value":"enum"},{"type":"space"},{"type":"name","value":"Entry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      A view into a single entry in a [`TableSeq`]'s subtable, which may either be vacant or occupied.
      
      This `enum` is constructed from the [`entry`] method on [`TableSeq`].
      
      

      .. rust:struct:: table_seq::table_seq::Entry::Vacant
         :index: 2
         :vis: pub
         :toc: Vacant
         :layout: [{"type":"name","value":"Vacant"},{"type":"punctuation","value":"("},{"type":"link","value":"VacantEntry","target":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         A vacant entry.

      .. rust:struct:: table_seq::table_seq::Entry::Occupied
         :index: 2
         :vis: pub
         :toc: Occupied
         :layout: [{"type":"name","value":"Occupied"},{"type":"punctuation","value":"("},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

         An occupied entry.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::entry::Entry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Entry","target":"Entry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Entry


         .. rubric:: Functions


         .. rust:function:: table_seq::table_seq::entry::Entry::and_modify
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"and_modify"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"f"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Self","target":"Self"}]

            Provides in-place mutable access to an occupied entry before any potential inserts into the `TableSeq`.

         .. rust:function:: table_seq::table_seq::entry::Entry::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Inserts an entry into the subtable at the hash value corresponding to the `Entry`, overwriting any existing value.
            
            If `value` does not hash to that hash value, the table is left in an indeterminate, but memory-safe state.

         .. rust:function:: table_seq::table_seq::entry::Entry::into_tables
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_tables"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Converts the `Entry` into a mutable reference to the underlying `TableSeq`.

         .. rust:function:: table_seq::table_seq::entry::Entry::or_insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or_insert"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"default"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Ensures a value is in the entry by inserting the default if empty, and returns an `OccupiedEntry`.

         .. rust:function:: table_seq::table_seq::entry::Entry::or_insert_with
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"or_insert_with"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"default"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnOnce","target":"FnOnce"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Ensures a value is in the entry by inserting the result of the default function if empty, and returns an `OccupiedEntry`.

         .. rust:function:: table_seq::table_seq::entry::Entry::subtable
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"subtable"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the subtable index of the `Entry`.

   .. rubric:: Structs and Unions


   .. rust:struct:: table_seq::table_seq::OccupiedEntry
      :index: 1
      :vis: pub
      :toc: struct OccupiedEntry
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      A view into an occupied entry in a [`TableSeq`]'s subtable.
      It is part of the [`Entry`] enum.
      
      

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::entry::OccupiedEntry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl OccupiedEntry


         .. rubric:: Functions


         .. rust:function:: table_seq::table_seq::entry::OccupiedEntry::get
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"}]

            Gets a reference to the value of the entry.

         .. rust:function:: table_seq::table_seq::entry::OccupiedEntry::get_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"get_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Gets a mutable reference to the value of the entry.

         .. rust:function:: table_seq::table_seq::entry::OccupiedEntry::into_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_mut"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Converts the `OccupiedEntry` into a mutable reference to the value of the entry.

         .. rust:function:: table_seq::table_seq::entry::OccupiedEntry::into_tables
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_tables"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Converts the `OccupiedEntry` into a mutable reference to the underlying `TableSeq`.

         .. rust:function:: table_seq::table_seq::entry::OccupiedEntry::remove
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"VacantEntry","target":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Removes the entry from the subtable, returning the value of the entry and a `VacantEntry` referring to the same slot.

         .. rust:function:: table_seq::table_seq::entry::OccupiedEntry::subtable
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"subtable"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the subtable index of the `OccupiedEntry`.

   .. rust:struct:: table_seq::table_seq::OwnedSubtable
      :index: 1
      :vis: pub
      :toc: struct OwnedSubtable
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"OwnedSubtable"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Structure holding an individually owned [`TableSeq`] subtable.
      
      While an [`TableSeq`] is optimized for reduced memory usage, this type always has the
      capacity to store up to 16 entries in-line and thus should only be used as temporary storage for
      passing or returning full subtables.
      
      

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::owned::OwnedSubtable
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"OwnedSubtable","target":"OwnedSubtable"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl OwnedSubtable


         .. rubric:: Functions


         .. rust:function:: table_seq::table_seq::owned::OwnedSubtable::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns whether this subtable is empty.

         .. rust:function:: table_seq::table_seq::owned::OwnedSubtable::iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SubtableIter","target":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator yielding references to all subtable entries.
            
            The subtable maintains a fixed iteration order that only changes with mutations. Beyond
            that, the iteration order is unspecified.

         .. rust:function:: table_seq::table_seq::owned::OwnedSubtable::iter_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"iter_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SubtableIterMut","target":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator yielding mutable references to all subtable entries.
            
            The subtable maintains a fixed iteration order that only changes with mutations. Beyond
            that, the iteration order is unspecified.

         .. rust:function:: table_seq::table_seq::owned::OwnedSubtable::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of entries in this subtable.

      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::table_seq::owned::OwnedSubtable::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"OwnedSubtable","target":"OwnedSubtable"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for OwnedSubtable


      .. rust:impl:: table_seq::table_seq::owned::OwnedSubtable::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"OwnedSubtable","target":"OwnedSubtable"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Default for OwnedSubtable


      .. rust:impl:: table_seq::table_seq::owned::OwnedSubtable::IntoIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"OwnedSubtable","target":"OwnedSubtable"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl IntoIterator for OwnedSubtable


   .. rust:struct:: table_seq::table_seq::SubtableIter
      :index: 1
      :vis: pub
      :toc: struct SubtableIter
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Iterator yielding references to a subtable's entries.
      
      

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::iter::SubtableIter
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtableIter","target":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl SubtableIter


      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::table_seq::iter::SubtableIter::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SubtableIter","target":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Default for SubtableIter


      .. rust:impl:: table_seq::table_seq::iter::SubtableIter::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SubtableIter","target":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for SubtableIter


      .. rust:impl:: table_seq::table_seq::iter::SubtableIter::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SubtableIter","target":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for SubtableIter


   .. rust:struct:: table_seq::table_seq::SubtableIterMut
      :index: 1
      :vis: pub
      :toc: struct SubtableIterMut
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Iterator yielding mutable references to a subtable's entries.
      
      

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::iter::SubtableIterMut
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"SubtableIterMut","target":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl SubtableIterMut


      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::table_seq::iter::SubtableIterMut::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SubtableIterMut","target":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Default for SubtableIterMut


      .. rust:impl:: table_seq::table_seq::iter::SubtableIterMut::Iterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Iterator","target":"Iterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SubtableIterMut","target":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Iterator for SubtableIterMut


      .. rust:impl:: table_seq::table_seq::iter::SubtableIterMut::ExactSizeIterator
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"ExactSizeIterator","target":"ExactSizeIterator"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"SubtableIterMut","target":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl ExactSizeIterator for SubtableIterMut


   .. rust:struct:: table_seq::table_seq::TableSeq
      :index: 1
      :vis: pub
      :toc: struct TableSeq
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      Indexed sequence of low-level hash tables with explicit hashing.
      
      This type serves as a memory and runtime efficient replacement for `Vec<HashTable<T>>`. In
      particular, it is optimized for the use-case where the vast majority of contained hash tables
      are tiny, each having 16 or fewer entries, while still allowing for a small but significant
      fraction of tables to be large.
      
      The provided API is loosely based on hashbrown's [`HashTable`] and on std's [`Vec`]. For methods
      that operate on individual entries, we include a `subtable` suffix or prefix to indicate
      `Vec`-level methods that operate on whole subtables instead of individual entries. For methods
      that operate on collections of entries, we include a `subtable` suffix or prefix to indicate
      `HashTable`-level methods that operate on an individual subtable instead of the collection of
      all subtables. For methods that operate on all entries of all subtables we include `flat` suffix
      or prefix.

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::TableSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl TableSeq


         .. rubric:: Functions


         .. rust:function:: table_seq::table_seq::TableSeq::clear
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"}]

            Discards all subtables with all their entries.

         .. rust:function:: table_seq::table_seq::TableSeq::clear_subtable
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"clear_subtable"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Discards all entries of a given subtable, leaving the subtable entry.
            
            The contained entries are dropped in place, see [`take_subtable`][Self::take_subtable] for a
            method that clears a subtable but returns the contained entries.

         .. rust:function:: table_seq::table_seq::TableSeq::find
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"hash"},{"type":"punctuation","value":": "},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"eq"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Finds an entry of a subtable using a given hash value and returns a reference to the found
            entry.
            
            This method calls `eq` to determine if a candidate entry should be returned. Any entry of
            the selected subtable is a potential argument for `eq`, but it will never be called with
            entries from other subtables.

         .. rust:function:: table_seq::table_seq::TableSeq::find_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"find_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"hash"},{"type":"punctuation","value":": "},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"eq"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Finds an entry of a subtable using a given hash value and returns a mutable reference to the
            found entry.
            
            This method calls `eq` to determine if a candidate entry should be returned. Any entry of
            the selected subtable is a potential argument for `eq`, but it will never be called with
            entries from other subtables.

         .. rust:function:: table_seq::table_seq::TableSeq::flat_len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"flat_len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the total number of entries across all subtables.
            
            See also [`len`][Self::len] and [`subtable_len`][Self::subtable_len].

         .. rust:function:: table_seq::table_seq::TableSeq::grow_for_subtable
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"grow_for_subtable"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Appends empty subtables up to and including the given subtable index.

         .. rust:function:: table_seq::table_seq::TableSeq::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"hash"},{"type":"punctuation","value":": "},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"eq"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":", "},{"type":"name","value":"hasher"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"}]

            Inserts an entry into a subtable using the given hash value.
            
            When an equivalent entry is already present, the subtable is not modified.
            
            Returns a pair containing a) mutable reference to an equivalent entry present in the table,
            either an existing or the newly inserted value and b) the passed value if an equivalent
            entry was already present.
            
            This method calls `eq` to determine if a candidate entry is equivalent. Any entry of the
            selected subtable is a potential argument for `eq`, but it will never be called with entries
            from other subtables.
            
            When switching representations or resizing large subtables, `hasher` will be called to
            re-compute hash values of the selected subtable. It will not be called for entries from
            other subtables.

         .. rust:function:: table_seq::table_seq::TableSeq::insert_unique
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert_unique"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"hash"},{"type":"punctuation","value":": "},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":", "},{"type":"name","value":"hasher"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"T","target":"T"}]

            Inserts an entry into a subtable using the given hash value, without checking for whether an
            equivalent entry is already present.
            
            Returns a mutable reference to the entry containing the newly inserted value.
            
            When switching representations or resizing large subtables, `hasher` will be called to
            re-compute hash values of the selected subtable. It will not be called for entries from
            other subtables.

         .. rust:function:: table_seq::table_seq::TableSeq::is_empty
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"is_empty"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"}]

            Returns `true` when this indexed table contains no subtables.

         .. rust:function:: table_seq::table_seq::TableSeq::len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of contained subtables.
            
            This does count empty subtables.
            
            See also [`subtable_len`][Self::subtable_len] and [`flat_len`][Self::flat_len].

         .. rust:function:: table_seq::table_seq::TableSeq::remove
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"remove"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":", "},{"type":"name","value":"hash"},{"type":"punctuation","value":": "},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":", "},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"name","value":"eq"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"FnMut","target":"FnMut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"bool","target":"bool"},{"type":"punctuation","value":", "},{"type":"name","value":"hasher"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"Option","target":"Option"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Finds and removes an entry from a subtable using the given hash value.
            
            When an equivalent entry is not found, the subtable is not modified, otherwise the value of
            the found entry is returned.
            
            This method calls `eq` to determine if a candidate entry is equivalent. Any entry of the
            selected subtable is a potential argument for `eq`, but it will never be called with entries
            from other subtables.
            
            When switching representations or resizing large subtables, `hasher` will be called to
            re-compute hash values of the selected subtable. It will not be called for entries
            from other subtables.

         .. rust:function:: table_seq::table_seq::TableSeq::resize
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"resize"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtables"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"}]

            Resizes the indexed table to a given number of subtables.
            
            When this is used to increase the number of subtables, empty subtables are appended at the
            end. When used to decrease the number of subtables, excess subtables at the end are
            discarded, including any entries they may have contained before.
            
            This is the primary way to create new empty subtables within an indexed table.
            
            # Examples
            
            ```should_panic
            use table_seq::TableSeq;
            let mut table: TableSeq<u64> = Default::default();
            let hasher = |&value: &u64| value.wrapping_mul(0x2545f4914f6cdd1d);
            
            // Subtables are not implicitly created, so this will panic!
            table.insert_unique(5, hasher(&42), 42, hasher);
            ```
            
            ```
            # use table_seq::TableSeq;
            # let mut table: TableSeq<u64> = Default::default();
            # let hasher = |&value: &u64| value.wrapping_mul(0x2545f4914f6cdd1d);
            #
            // Initializing subtables with `resize` avoids this panic
            table.resize(10);
            table.insert_unique(5, hasher(&42), 42, hasher);
            table.insert_unique(5, hasher(&43), 43, hasher);
            table.insert_unique(6, hasher(&44), 44, hasher);
            
            assert_eq!(table.len(), 10);
            assert_eq!(table.flat_len(), 3);
            
            // Reducing the number of subtables also removes their entries
            table.resize(8);
            assert_eq!(table.len(), 8);
            assert_eq!(table.flat_len(), 3);
            
            table.resize(6);
            assert_eq!(table.len(), 6);
            assert_eq!(table.flat_len(), 2);
            
            table.resize(2);
            assert_eq!(table.len(), 2);
            assert_eq!(table.flat_len(), 0);
            ```
            

         .. rust:function:: table_seq::table_seq::TableSeq::subtable_iter
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"subtable_iter"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SubtableIter","target":"SubtableIter"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator yielding references for all entries of a subtable.
            
            Each subtable maintains a fixed iteration order that only changes with mutations. Beyond
            that, the iteration order is unspecified.

         .. rust:function:: table_seq::table_seq::TableSeq::subtable_iter_mut
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"subtable_iter_mut"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"SubtableIterMut","target":"SubtableIterMut"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Returns an iterator yielding mutable references for all entries of a subtable.
            
            Each subtable maintains a fixed iteration order that only changes with mutations. Beyond
            that, the iteration order is unspecified.

         .. rust:function:: table_seq::table_seq::TableSeq::subtable_len
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"subtable_len"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the number of entries of a given subtable.
            
            See also [`len`][Self::len] and [`flat_len`][Self::flat_len].

         .. rust:function:: table_seq::table_seq::TableSeq::take_subtable
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"take_subtable"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"subtable"},{"type":"punctuation","value":": "},{"type":"link","value":"usize","target":"usize"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OwnedSubtable","target":"OwnedSubtable"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Removes and returns all entries from a subtable, leaving an empty subtable.
            
            See [`clear_subtable`][Self::take_subtable] for a method that clears a subtable while
            dropping all entries in place.

      .. rust:impl:: table_seq::table_seq::TableSeq
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl TableSeq


      .. rubric:: Traits implemented


      .. rust:impl:: table_seq::table_seq::TableSeq::Send
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Send","target":"Send"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Send","target":"Send"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Send for TableSeq


      .. rust:impl:: table_seq::table_seq::TableSeq::Sync
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"unsafe"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Sync","target":"Sync"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Sync","target":"Sync"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Sync for TableSeq


      .. rust:impl:: table_seq::table_seq::TableSeq::Debug
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"fmt","target":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Debug"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Debug for TableSeq


      .. rust:impl:: table_seq::table_seq::TableSeq::Default
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Default","target":"Default"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Default for TableSeq


      .. rust:impl:: table_seq::table_seq::TableSeq::Drop
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"Drop","target":"Drop"},{"type":"space"},{"type":"keyword","value":"for"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl Drop for TableSeq


   .. rust:struct:: table_seq::table_seq::VacantEntry
      :index: 1
      :vis: pub
      :toc: struct VacantEntry
      :layout: [{"type":"keyword","value":"struct"},{"type":"space"},{"type":"name","value":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"}]

      A view into a vacant entry in a [`TableSeq`]'s subtable.
      It is part of the [`Entry`] enum.
      
      

      .. rubric:: Implementations


      .. rust:impl:: table_seq::table_seq::entry::VacantEntry
         :index: -1
         :vis: pub
         :layout: [{"type":"keyword","value":"impl"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"name","value":"T"},{"type":"punctuation","value":">"},{"type":"space"},{"type":"link","value":"VacantEntry","target":"VacantEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]
         :toc: impl VacantEntry


         .. rubric:: Functions


         .. rust:function:: table_seq::table_seq::entry::VacantEntry::insert
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"insert"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":", "},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"OccupiedEntry","target":"OccupiedEntry"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'a"},{"type":"punctuation","value":", "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Inserts an entry into the subtable at the hash value corresponding to the `VacantEntry`.
            
            If `value` does not hash to that hash value, the table is left in an indeterminate, but memory-safe state.

         .. rust:function:: table_seq::table_seq::entry::VacantEntry::into_tables
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"into_tables"},{"type":"punctuation","value":"("},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"punctuation","value":"&"},{"type":"lifetime","value":"'a"},{"type":"space"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"TableSeq","target":"TableSeq"},{"type":"punctuation","value":"<"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":">"}]

            Converts the `VacantEntry` into a mutable reference to the underlying `TableSeq`.

         .. rust:function:: table_seq::table_seq::entry::VacantEntry::subtable
            :index: -1
            :vis: pub
            :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"subtable"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"self"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"usize","target":"usize"}]

            Returns the subtable index of the `VacantEntry`.
