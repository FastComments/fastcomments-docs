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

By navigating [여기](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments 가져오기 페이지로, 제공업체 선택 및 내보내기 파일 업로드 필드가 있습니다.'; title='가져오기 페이지 양식' app-screenshot-end]

### 가져오기 모니터링

FastComments는 가져오기와 내보내기를 처리하기 위해 작업 처리 시스템을 사용합니다. 시스템이 작업을 가져오면, 가져오기 또는 내보내기 UI에서 작업 상태를 주기적으로 보고합니다.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='실행 중인 가져오기 작업과 작업 처리 시스템이 보고하는 상태를 보여주는 가져오기 페이지'; title='가져오기 작업 상태' app-screenshot-end]

Note that the status for Imports and Export are viewable by all administrators in the account.

If your job fails, it will not automatically be restarted. The import will have to be attempted again. If any import or export fails, our system administrators are automatically notified. If we identify an issue, we'll reach out to you to see if we can help.

### 가져오기 재실행

During some migrations, it is necessary to run the import multiple times. For example, it is common to do a first pass migration for testing, and then run the import again with the latest data before flipping the switch.

Re-importing the same content **will not create duplicates**.

### 데이터 보안 및 만료

Import files are not accessible via outside requests in any way, and import files are deleted from our system as soon as the import completes.

---