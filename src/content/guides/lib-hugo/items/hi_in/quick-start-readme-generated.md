अपनी tenant ID एक बार `hugo.toml` में सेट करें:

```toml
[params.fastcomments]
  tenantId = "demo"   # "demo" को अपनी FastComments tenant ID से बदलें
```

फिर या तो कमेंट्स विजेट को अपनी थीम में जोड़ें ([Theme Integration](#theme-integration-readme-generated) देखें), या किसी भी पेज के Markdown में एक shortcode डालें:

```text
\{{< fastcomments >}}
```