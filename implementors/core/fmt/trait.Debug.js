(function() {var implementors = {};
implementors["starchart"] = [{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> + 'static&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/error/enum.ActionError.html\" title=\"enum starchart::error::ActionError\">ActionError</a>&lt;E&gt;","synthetic":false,"types":["starchart::action::error::ActionError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/error/enum.ActionValidationError.html\" title=\"enum starchart::error::ActionValidationError\">ActionValidationError</a>","synthetic":false,"types":["starchart::action::error::ActionValidationError"]},{"text":"impl&lt;E:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/error/enum.ActionRunError.html\" title=\"enum starchart::error::ActionRunError\">ActionRunError</a>&lt;E&gt;","synthetic":false,"types":["starchart::action::error::ActionRunError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/action/struct.CreateOperation.html\" title=\"struct starchart::action::CreateOperation\">CreateOperation</a>","synthetic":false,"types":["starchart::action::impl::CreateOperation"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/action/struct.ReadOperation.html\" title=\"struct starchart::action::ReadOperation\">ReadOperation</a>","synthetic":false,"types":["starchart::action::impl::ReadOperation"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/action/struct.UpdateOperation.html\" title=\"struct starchart::action::UpdateOperation\">UpdateOperation</a>","synthetic":false,"types":["starchart::action::impl::UpdateOperation"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/action/struct.DeleteOperation.html\" title=\"struct starchart::action::DeleteOperation\">DeleteOperation</a>","synthetic":false,"types":["starchart::action::impl::DeleteOperation"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/action/struct.TableTarget.html\" title=\"struct starchart::action::TableTarget\">TableTarget</a>","synthetic":false,"types":["starchart::action::impl::TableTarget"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/action/struct.EntryTarget.html\" title=\"struct starchart::action::EntryTarget\">EntryTarget</a>","synthetic":false,"types":["starchart::action::impl::EntryTarget"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/action/enum.ActionKind.html\" title=\"enum starchart::action::ActionKind\">ActionKind</a>","synthetic":false,"types":["starchart::action::kind::ActionKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/action/enum.OperationTarget.html\" title=\"enum starchart::action::OperationTarget\">OperationTarget</a>","synthetic":false,"types":["starchart::action::target::OperationTarget"]},{"text":"impl&lt;S:&nbsp;<a class=\"trait\" href=\"starchart/trait.Entry.html\" title=\"trait starchart::Entry\">Entry</a>, C:&nbsp;<a class=\"trait\" href=\"starchart/action/trait.CrudOperation.html\" title=\"trait starchart::action::CrudOperation\">CrudOperation</a>, T:&nbsp;<a class=\"trait\" href=\"starchart/action/trait.OpTarget.html\" title=\"trait starchart::action::OpTarget\">OpTarget</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/struct.Action.html\" title=\"struct starchart::Action\">Action</a>&lt;S, C, T&gt;","synthetic":false,"types":["starchart::action::Action"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.BincodeBackend.html\" title=\"struct starchart::backend::BincodeBackend\">BincodeBackend</a>","synthetic":false,"types":["starchart::backend::bincode::BincodeBackend"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/backend/fs/enum.FsError.html\" title=\"enum starchart::backend::fs::FsError\">FsError</a>","synthetic":false,"types":["starchart::backend::fs::FsError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.JsonBackend.html\" title=\"struct starchart::backend::JsonBackend\">JsonBackend</a>","synthetic":false,"types":["starchart::backend::json::JsonBackend"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.JsonPrettyBackend.html\" title=\"struct starchart::backend::JsonPrettyBackend\">JsonPrettyBackend</a>","synthetic":false,"types":["starchart::backend::json::JsonPrettyBackend"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/error/enum.MemoryError.html\" title=\"enum starchart::error::MemoryError\">MemoryError</a>","synthetic":false,"types":["starchart::backend::memory::MemoryError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.MemoryBackend.html\" title=\"struct starchart::backend::MemoryBackend\">MemoryBackend</a>","synthetic":false,"types":["starchart::backend::memory::MemoryBackend"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.TomlBackend.html\" title=\"struct starchart::backend::TomlBackend\">TomlBackend</a>","synthetic":false,"types":["starchart::backend::toml::TomlBackend"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.TomlPrettyBackend.html\" title=\"struct starchart::backend::TomlPrettyBackend\">TomlPrettyBackend</a>","synthetic":false,"types":["starchart::backend::toml::TomlPrettyBackend"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/backend/struct.YamlBackend.html\" title=\"struct starchart::backend::YamlBackend\">YamlBackend</a>","synthetic":false,"types":["starchart::backend::yaml::YamlBackend"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"enum\" href=\"starchart/enum.Error.html\" title=\"enum starchart::Error\">ChartError</a>&lt;B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"associatedtype\" href=\"starchart/backend/trait.Backend.html#associatedtype.Error\" title=\"type starchart::backend::Backend::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a>,&nbsp;</span>","synthetic":false,"types":["starchart::error::ChartError"]},{"text":"impl&lt;B:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"starchart/backend/trait.Backend.html\" title=\"trait starchart::backend::Backend\">Backend</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"starchart/struct.Starchart.html\" title=\"struct starchart::Starchart\">Starchart</a>&lt;B&gt;","synthetic":false,"types":["starchart::starchart::Starchart"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()