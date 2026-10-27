---
虽然它的名称是域迁移工具，但它也可以用于在单个页面之间迁移评论。

只需在 `from` 和 `to` 输入字段中输入旧的和新的 [urlId](/guide-customizations-and-configuration.html#url-id) 值。

### 通过 API 迁移页面

要一次迁移多个页面，请使用 [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post)。每个请求最多可包含 100 对 `from` 和 `to`，并将它们作为批处理作业运行。

将 `type` 设置为 `urlId` 时，仅移动 `urlId` 完全等于 `from` 的页面，因此 `post-1` 不会匹配 `post-10`。仪表板工具没有此选项。

每个 API 迁移在 90 天内都可以通过 [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post) 撤销。

---