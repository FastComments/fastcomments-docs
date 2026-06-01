---
在 `hugo.toml` 中設定您的租戶 ID（只需設定一次）:

```toml
[params.fastcomments]
  tenantId = "demo"   # 將 "demo" 替換為您的 FastComments 租戶 ID
```

然後您可以將評論小工具接入您的主題（參見 [主題整合](#theme-integration-readme-generated)），或在任何頁面的 Markdown 中放入一個 shortcode:

```text
\{{< fastcomments >}}
```
---