===========
``mod fmt``
===========


.. rust:module:: imctk_util::fmt
   :index: 0
   :vis: pub


   .. rust:use:: imctk_util::fmt
      :used_name: self


   .. rust:use:: std::fmt::Display
      :used_name: Display


   .. rust:use:: std::fmt::Formatter
      :used_name: Formatter


   .. rust:use:: std::fmt::Debug
      :used_name: Debug


   .. rubric:: Functions


   .. rust:function:: imctk_util::fmt::fmt_closure
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"fmt_closure"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":"("},{"type":"punctuation","value":"&"},{"type":"keyword","value":"mut"},{"type":"space"},{"type":"link","value":"Formatter","target":"Formatter"},{"type":"punctuation","value":"<"},{"type":"lifetime","value":"'_"},{"type":"punctuation","value":">"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"std","target":"std"},{"type":"punctuation","value":"::"},{"type":"name","value":"fmt"},{"type":"punctuation","value":"::"},{"type":"name","value":"Result"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"closure"},{"type":"punctuation","value":": "},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Display","target":"Display"},{"type":"punctuation","value":" + "},{"type":"link","value":"Debug","target":"Debug"}]

      Turns a closure that writes to a [`Formatter`] into a type that implements [`Display`] and
      [`Debug`] by calling that closure.

   .. rust:function:: imctk_util::fmt::fmt_list
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"fmt_list"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"get_iter"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Display","target":"Display"},{"type":"punctuation","value":" + "},{"type":"link","value":"Debug","target":"Debug"}]

      Takes an iterator returning closure and returns a value that formats the yielded items using
      [`Formatter::debug_list`] when formatted.

   .. rust:function:: imctk_util::fmt::fmt_map
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"fmt_map"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"punctuation","value":"("},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"punctuation","value":", "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"punctuation","value":")"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"get_iter"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Display","target":"Display"},{"type":"punctuation","value":" + "},{"type":"link","value":"Debug","target":"Debug"}]

      Takes an iterator returning closure and returns a value that formats the yielded items using
      [`Formatter::debug_set`] when formatted.

   .. rust:function:: imctk_util::fmt::fmt_set
      :index: 0
      :vis: pub
      :layout: [{"type":"keyword","value":"fn"},{"type":"space"},{"type":"name","value":"fmt_set"},{"type":"punctuation","value":"<"},{"type":"name","value":"T"},{"type":"punctuation","value":": "},{"type":"link","value":"IntoIterator","target":"IntoIterator"},{"type":"punctuation","value":"<"},{"type":"name","value":"Item"},{"type":"punctuation","value":" = "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Debug","target":"Debug"},{"type":"punctuation","value":">"},{"type":"punctuation","value":">"},{"type":"punctuation","value":"("},{"type":"name","value":"get_iter"},{"type":"punctuation","value":": "},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Fn","target":"Fn"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"link","value":"T","target":"T"},{"type":"punctuation","value":")"},{"type":"space"},{"type":"returns"},{"type":"space"},{"type":"keyword","value":"impl"},{"type":"space"},{"type":"link","value":"Display","target":"Display"},{"type":"punctuation","value":" + "},{"type":"link","value":"Debug","target":"Debug"}]

      Takes an iterator returning closure and returns a value that formats the yielded items using
      [`Formatter::debug_set`] when formatted.
