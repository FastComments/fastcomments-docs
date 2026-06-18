---
在 `hugo.toml` 中设置一次您的租户 ID:

```toml
[params.fastcomments]
  tenantId = "demo"   # 将 "demo" 替换为您的 FastComments 租户 ID
```

然后将评论小部件接入到您的主题（参见 [主题集成](#theme-integration-readme-generated)），或在任意页面的 Markdown 中插入短代码：

```text
\{{< fastcomments >}}
```
---