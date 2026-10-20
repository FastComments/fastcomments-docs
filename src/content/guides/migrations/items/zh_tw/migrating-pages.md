---
雖然它的名稱是 Domain Migration 工具，但它也可用於在單個頁面之間遷移評論。

只需在 `from` 與 `to` 輸入欄位中輸入舊的與新的 [urlId](/guide-customizations-and-configuration.html#url-id) 值。

### 透過 API 遷移頁面

若要一次遷移多個頁面，請使用 [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post)。每個請求最多可接受 100 組 `from` 與 `to` 配對，並將它們作為批次作業執行。

當 `type` 設為 `urlId` 時，僅會移動 `urlId` 完全等於 `from` 的頁面，因此 `post-1` 不會匹配 `post-10`。儀表板工具沒有此選項。

每個 API 遷移可在 90 天內使用 [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post) 進行復原。

---