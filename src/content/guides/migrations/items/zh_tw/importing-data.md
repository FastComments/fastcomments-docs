---
雖然 FastComments 支援團隊可以協助遷移，但大多數遷移都可以輕鬆自行執行並監控，無需支援人員介入。

我們原生支援從以下服務提供者匯入匯出資料：

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

透過前往 [此處](https://fastcomments.com/auth/my-account/manage-data/import) 我們可以上傳包含遷移資料的檔案。

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='匯入頁面表單' app-screenshot-end]

### 監控匯入

FastComments 使用工作處理系統來處理匯入與匯出。系統接收到您的工作後，會定期在匯入或匯出 UI 中報告工作狀態。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='匯入工作狀態' app-screenshot-end]

請注意，匯入與匯出的狀態可由帳號中的所有管理員檢視。

如果您的工作失敗，系統不會自動重新啟動。必須再次嘗試匯入。如果任何匯入或匯出失敗，我們的系統管理員會自動收到通知。若我們發現問題，會聯繫您以了解是否能提供協助。

### 重新執行匯入

在某些遷移過程中，需要多次執行匯入。例如，常見的做法是先進行一次測試性的遷移，然後在正式切換前使用最新資料再次執行匯入。

重新匯入相同內容 **不會產生重複**。

### 資料安全與過期

匯入檔案不會以任何方式透過外部請求存取，且匯入完成後，匯入檔案會立即從我們的系統中刪除。

---