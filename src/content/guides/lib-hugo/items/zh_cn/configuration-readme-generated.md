所有 FastComments 小部件选项在 `hugo.toml` 的 `[params.fastcomments]` 下设置，并且可以在页面的 front matter 的 `[fastcomments]` 下进行每页覆盖。优先级从低到高：站点参数、页面 front matter、短代码参数。

```toml
# hugo.toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  hasDarkBackground = true
  voteStyle = 1
  enableSearch = true
```

```toml
# a page's front matter
+++
title = "Article"
[fastcomments]
  urlId = "article-42"
  collapseReplies = true
+++
```

当既未提供 `url` 也未提供 `urlId` 时，`url` 会默认设为页面的永久链接 so comment threads stay tied to a stable URL。

### 欧盟数据驻留

欧盟客户将 `region = "eu"` 设置为将小部件路由到 `cdn-eu.fastcomments.com`：

```toml
[params.fastcomments]
  tenantId = "your-tenant-id"
  region = "eu"
```

### 键大小写说明

Hugo 在 `hugo.toml` 和 front matter 中会将每个键都转换为小写，但 FastComments 小部件需要 camelCase 的键（`tenantId`、`hasDarkBackground`）。该组件会自动为所有已知的顶级选项恢复正确的大小写，因此请以正常的 camelCase 形式编写选项。位于对象值内部的嵌套键（例如 `translations` 映射的键，或 `pageReactConfig` 的字段）不会被恢复。请通过 FastComments 仪表板的自定义界面来配置这些内容。