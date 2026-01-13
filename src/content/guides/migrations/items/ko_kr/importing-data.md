FastComments 지원팀이 마이그레이션을 도와줄 수 있지만, 대부분은 지원 담당자의 개입 없이도 간단히 수행하고 모니터링할 수 있습니다.

우리는 다음 제공업체에서의 내보내기 파일 가져오기를 기본적으로 지원합니다:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)

By navigating [여기](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### 가져오기 모니터링

FastComments는 가져오기 및 내보내기를 처리하기 위해 작업(job) 처리 시스템을 사용합니다. 시스템이 작업을 가져가면, 가져오기 또는 내보내기 UI에 작업 상태를 주기적으로 보고합니다.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

가져오기 및 내보내기 상태는 계정 내 모든 관리자들이 볼 수 있습니다.

작업이 실패하면 자동으로 재시작되지 않습니다. 가져오기는 다시 시도해야 합니다. 어떤 가져오기나 내보내기가 실패하면, 시스템 관리자가 자동으로 알림을 받습니다. 문제가 확인되면 도와드릴 수 있는지 확인하기 위해 연락드리겠습니다.

### 가져오기 재실행

일부 마이그레이션에서는 가져오기를 여러 번 실행해야 할 때가 있습니다. 예를 들어, 테스트를 위해 첫 번째 마이그레이션을 수행한 다음, 전환하기 전에 최신 데이터로 다시 가져오기를 실행하는 것이 일반적입니다.

같은 콘텐츠를 다시 가져와도 **중복이 생성되지 않습니다**.

### 데이터 보안 및 만료

가져오기 파일은 외부 요청으로 접근할 수 없으며, 가져오기가 완료되면 가져오기 파일은 즉시 시스템에서 삭제됩니다.