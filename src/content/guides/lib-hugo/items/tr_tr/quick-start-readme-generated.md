tenant ID'nizi `hugo.toml` dosyasında bir kez ayarlayın:

```toml
[params.fastcomments]
  tenantId = "demo"   # "demo" değerini FastComments tenant ID'niz ile değiştirin
```

Sonra ya yorumlar widget'ını temanıza bağlayın (bkz. [Tema Entegrasyonu](#theme-integration-readme-generated)), ya da herhangi bir sayfanın Markdown'una bir shortcode ekleyin:

```text
\{{< fastcomments >}}
```