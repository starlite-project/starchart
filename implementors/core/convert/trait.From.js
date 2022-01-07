(function() {var implementors = {};
implementors["starchart"] = [{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + 'static&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/error/enum.ActionRunError.html\" title=\"enum starchart::error::ActionRunError\">ActionRunError</a>&lt;E&gt;&gt; for <a class=\"enum\" href=\"starchart/error/enum.ActionError.html\" title=\"enum starchart::error::ActionError\">ActionError</a>&lt;E&gt;","synthetic":false,"types":["starchart::action::error::ActionError"]},{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + 'static&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/error/enum.ActionValidationError.html\" title=\"enum starchart::error::ActionValidationError\">ActionValidationError</a>&gt; for <a class=\"enum\" href=\"starchart/error/enum.ActionError.html\" title=\"enum starchart::error::ActionError\">ActionError</a>&lt;E&gt;","synthetic":false,"types":["starchart::action::error::ActionError"]},{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;E&gt; for <a class=\"enum\" href=\"starchart/error/enum.ActionRunError.html\" title=\"enum starchart::error::ActionRunError\">ActionRunError</a>&lt;E&gt;","synthetic":false,"types":["starchart::action::error::ActionRunError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"starchart/backend/fs/enum.FsError.html\" title=\"enum starchart::backend::fs::FsError\">FsError</a>","synthetic":false,"types":["starchart::backend::fs::FsError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/serde-value/0.7.0/serde_value/ser/enum.SerializerError.html\" title=\"enum serde_value::ser::SerializerError\">SerializerError</a>&gt; for <a class=\"enum\" href=\"starchart/error/enum.MemoryError.html\" title=\"enum starchart::error::MemoryError\">MemoryError</a>","synthetic":false,"types":["starchart::backend::memory::MemoryError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/serde-value/0.7.0/serde_value/de/enum.DeserializerError.html\" title=\"enum serde_value::de::DeserializerError\">DeserializerError</a>&gt; for <a class=\"enum\" href=\"starchart/error/enum.MemoryError.html\" title=\"enum starchart::error::MemoryError\">MemoryError</a>","synthetic":false,"types":["starchart::backend::memory::MemoryError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/error/enum.MemoryError.html\" title=\"enum starchart::error::MemoryError\">MemoryError</a>&gt; for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt;","synthetic":false,"types":["starchart::error::ChartError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/backend/fs/enum.FsError.html\" title=\"enum starchart::backend::fs::FsError\">FsError</a>&gt; for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt;","synthetic":false,"types":["starchart::error::ChartError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/error/enum.ActionValidationError.html\" title=\"enum starchart::error::ActionValidationError\">ActionValidationError</a>&gt; for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt;","synthetic":false,"types":["starchart::error::ChartError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/error/enum.ActionRunError.html\" title=\"enum starchart::error::ActionRunError\">ActionRunError</a>&lt;&lt;B as <a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt;::<a class=\"associatedtype\" href=\"starchart/backend/trait.Backend.html#associatedtype.Error\" title=\"type starchart::backend::Backend::Error\">Error</a>&gt;&gt; for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt;","synthetic":false,"types":["starchart::error::ChartError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html\" title=\"struct alloc::boxed::Box\">Box</a>&lt;dyn <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt;","synthetic":false,"types":["starchart::error::ChartError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"starchart/error/enum.ActionError.html\" title=\"enum starchart::error::ActionError\">ActionError</a>&lt;&lt;B as <a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt;::<a class=\"associatedtype\" href=\"starchart/backend/trait.Backend.html#associatedtype.Error\" title=\"type starchart::backend::Backend::Error\">Error</a>&gt;&gt; for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt;","synthetic":false,"types":["starchart::error::ChartError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()