雖然 FastComments 支援可以協助遷移，但大多數遷移可以在不需支援人員介入的情況下輕鬆完成並加以監控。

我們原生支援從下列提供者匯入匯出資料：

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)

透過前往 [此處](https://fastcomments.com/auth/my-account/manage-data/import) 我們可以上傳包含欲遷移資料的檔案。

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='匯入頁面表單' app-screenshot-end]

### 監控匯入

FastComments 使用一個工作處理系統來處理匯入與匯出。一旦系統開始處理您的工作，它會在匯入或匯出使用者介面中定期回報該工作的狀態。

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='匯入作業狀態' app-screenshot-end]

請注意，匯入與匯出之狀態可由帳戶中所有管理員檢視。

若您的工作失敗，系統不會自動重新啟動。需再次嘗試匯入。若任何匯入或匯出失敗，我們的系統管理員會自動收到通知。若我們發現問題，將會聯絡您，看看是否能提供協助。

### 重新執行匯入

在某些遷移情境中，需要多次執行匯入。例如，常見作法是先執行第一輪遷移以進行測試，然後在正式切換前再以最新資料重新執行匯入。

重新匯入相同內容 **不會產生重複**。

### 資料安全與過期

匯入檔案不會以任何方式對外可存取，且匯入完成後匯入檔案會立即從系統中刪除。

---