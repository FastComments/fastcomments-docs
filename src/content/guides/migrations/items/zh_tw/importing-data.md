雖然 FastComments 支援可以協助遷移，但大多數遷移可以在不需支援人員介入的情況下輕鬆執行與監控。

我們原生支援從下列提供者匯入匯出資料：

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

透過前往 [此處](https://fastcomments.com/auth/my-account/manage-data/import) 我們可以上傳包含要遷移資料的檔案。

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### 監控匯入

FastComments 使用工作處理系統來處理匯入和匯出。一旦系統接手你的工作，它會
定期在匯入或匯出的使用者介面中回報該工作的狀態。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

請注意，匯入與匯出狀態可由帳號中的所有管理員檢視。

如果你的工作失敗，它不會自動重新啟動。匯入必須再次嘗試。如果任何匯入或匯出失敗，
我們的系統管理員會自動收到通知。如果我們發現問題，我們會與您聯繫，看看是否能提供協助。

### 重新執行匯入

在某些遷移過程中，可能需要多次執行匯入。例如，常見的做法是先進行第一次
測試性遷移，然後在正式切換前使用最新資料再次執行匯入。

重新匯入相同內容 **不會產生重複**。

### 資料安全與過期

匯入檔案不會以任何方式對外部請求開放，且匯入檔案會在
匯入完成後立即從我們的系統中刪除。

---