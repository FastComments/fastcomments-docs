配置来自三个来源。较后的来源优先：

1. **全局默认值** 位于 `_config.yml` 的 `fastcomments:` 键下。
2. **页面上下文**，为页面作用域的小部件自动派生（见下文）。
3. **标签属性** 写在标签自身上。

因此，标签上的 `url_id` 会覆盖页面派生值，而页面派生值会覆盖任何全局默认值。

### 属性语法

属性是以 `snake_case` 的 `key=value` 对：

```liquid
{% raw %}{% fastcomments url_id="my-stable-id" readonly=true count=20 %}{% endraw %}
```

- **带引号的** 值 (`"..."` 或 `'...'`) 是字面字符串。
- **不带引号的** `true`/`false` 会成为布尔值，数字会成为数字。
- **不带引号的** 其他任何内容会作为页面上下文中的 Liquid 变量解析，例如 `url_id=page.slug`。 (Liquid 不会在标签的属性内展开 `{% raw %}\{{ ... }}{% endraw %}`，因此请使用裸形式 `page.slug`，而不是 `"{% raw %}\{{ page.slug }}{% endraw %}"`。)

snake_case 的属性和配置键会被自动映射到 FastComments 所期望的 camelCase 键（`tenant_id` → `tenantId`，`url_id` → `urlId`，`page_title` → `pageTitle`，`has_dark_background` → `hasDarkBackground`，等等）。来自 [小部件配置](https://docs.fastcomments.com/guide-customizations-and-configuration.html) 的任何其他选项也会以相同方式直接传递。

### 页面派生值

对于页面作用域的小部件（`fastcomments`, `fastcomments_comment_count`, `fastcomments_live_chat`, `fastcomments_collab_chat`, `fastcomments_image_chat`）这些值会自动从当前页面填充，除非您自行设置：

- `url_id` ← `page.url` （一个独立于访问域名的稳定标识符）
- `url` ← `site.url` + `page.url` （仅当在 `_config.yml` 中设置了 `url` 时）
- `page_title` ← `page.title`

站点范围的小部件（最近评论/讨论、热门页面、评论汇总、用户活动提要、批量计数）不与页面绑定，也不会派生这些值。

### 欧盟数据驻留

欧盟客户添加 `region: eu`，可全局设置：

```yaml
fastcomments:
  tenant_id: your-tenant-id
  region: eu
```

或按标签设置：`{% raw %}{% fastcomments region="eu" %}{% endraw %}`。小部件随后将从欧盟 CDN 加载。