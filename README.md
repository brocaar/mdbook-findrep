# mdbook-findrep

`mdbook-findrep` is a very simple pre-processor for replacing variables in an [mdBook](https://rust-lang.github.io/mdBook/).
Please note that the current version is compatible with mdBook v0.4.

```toml
[preprocessor.findrep]
    foo = "bar"
```

With the above configuration, it would replace any `$FOO` in the documentation with `bar`.
Please note that keys are automatically upper-cased and prefixed with a `$`.
