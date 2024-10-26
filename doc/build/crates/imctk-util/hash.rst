============
``mod hash``
============


.. rust:module:: imctk_util::hash
   :index: 0
   :vis: pub


   .. rust:use:: imctk_util::hash
      :used_name: self


   .. rust:use:: std::hash::BuildHasherDefault
      :used_name: BuildHasherDefault


   .. rust:use:: std::hash::BuildHasher
      :used_name: BuildHasher


   .. rust:use:: zwohash::ZwoHasher
      :used_name: ZwoHasher


   .. rubric:: Functions


   .. rust:function:: imctk_util::hash::hash_ref
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"hash_ref"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"hash"},{"type":"punctuation","value":"::"},{"type":"name","value":"Hash"},{"type":"punctuation","value":" + "},{"type":"punctuation","value":"?"},{"type":"link","value":"Sized","target":"Sized"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"punctuation","value":"&"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]

      Computes the hash of a reference using imctk's default hasher.
      
      This forwards to [`hash_value`]. Restricting the argument to be a reference is occasionally
      useful for type inference or for avoiding warnings.

   .. rust:function:: imctk_util::hash::hash_value
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"hash_value"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"hash"},{"type":"punctuation","value":"::"},{"type":"name","value":"Hash"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"value"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"u64","target":"u64"}]

      Computes the hash of a value using imctk's default hasher.
