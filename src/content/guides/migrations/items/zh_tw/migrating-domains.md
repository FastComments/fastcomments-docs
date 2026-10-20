FastComments 提供了一種自動化方式，讓您可以在不同網域之間遷移評論。

網域遷移只需要一個 `from` 網域和一個 `to` 網域。

此操作 **移動** 評論，並不會複製它們。如果您希望複製評論，請與我們聯繫。

[app-screenshot-start url='/auth/my-account/manage-data/migrate-domains?demo=true'; linkUrl='/auth/my-account/manage-data/migrate-domains'; selector = '.content'; alt='具有來源與目標網域欄位以及已遷移評論數量的網域遷移工具'; title='遷移網域' app-screenshot-end]

這在某些情況下也很有用，例如如果您遷移到 FastComments 的過程中包含從其他提供者遷移，您的評論匯入
資料可能包含需要遷移的資料。在此情況下，您可以先執行匯入，然後再進行網域遷移。

### 監控進度

網域遷移工具使用與其他資料管理工具相同的 FastComments 工作處理系統。

在遷移開始之前可能會有延遲。這是正常的，因為系統會定期檢查是否有新工作需要處理。

當工作執行時，系統會顯示已找到的待遷移評論數量，以及目前已遷移的評論數量。

### 透過 API 遷移網域

也可以使用 [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post) 並將 `type` 設為 `domain` 來遷移網域。可從 [`GET /api/v1/batch-jobs/:id`](/guide-api.html#batch-job-get-by-id) 取得進度，且遷移可在 90 天內撤銷。

---