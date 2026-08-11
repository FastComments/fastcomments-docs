While FastComments Support can help with migrations, most can be performed and monitored easily without any intervention of support staff.

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

By navigating [此處](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments 匯入頁面，包含提供者選擇與匯出檔案的檔案上傳欄位'; title='匯入頁面表單' app-screenshot-end]

### 監控匯入

FastComments uses a job processing system for processing imports and exports. Once the system has picked up your job, it will periodically report the status of the job in the import or export UI.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='匯入頁面顯示執行中的匯入工作以及工作處理系統回報的狀態'; title='匯入工作狀態' app-screenshot-end]

Note that the status for Imports and Export are viewable by all administrators in the account.

If your job fails, it will not automatically be restarted. The import will have to be attempted again. If any import or export fails, our system administrators are automatically notified. If we identify an issue, we'll reach out to you to see if we can help.

### 重新執行匯入

During some migrations, it is necessary to run the import multiple times. For example, it is common to do a first pass migration for testing, and then run the import again with the latest data before flipping the switch.

Re-importing the same content **will not create duplicates**.

### 資料安全與過期

Import files are not accessible via outside requests in any way, and import files are deleted from our system as soon as the import completes.

---