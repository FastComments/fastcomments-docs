---
FastComments 지원팀이 마이그레이션을 도와줄 수는 있지만, 대부분은 지원 인력의 개입 없이도 쉽게 수행 및 모니터링할 수 있습니다.
지원 스태프의 개입 없이도 가능합니다.

We natively support importing exports from the following providers:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- WordPress (via the plugin)
- AnyComment (Via WordPress Import/Export)

By navigating [여기](https://fastcomments.com/auth/my-account/manage-data/import) we can upload the file containing the data to migrate.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; title='The Import Page Form' app-screenshot-end]

### 임포트 모니터링

FastComments는 임포트 및 익스포트를 처리하기 위해 작업 처리 시스템을 사용합니다. 시스템이 귀하의 작업을 가져가면,
임포트 또는 익스포트 UI에 작업 상태를 주기적으로 보고합니다.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; title='Import Job Status' app-screenshot-end]

임포트 및 익스포트의 상태는 계정의 모든 관리자가 볼 수 있습니다.

작업이 실패하면 자동으로 재시작되지 않습니다. 임포트를 다시 시도해야 합니다. 어떤 임포트나 익스포트가 실패하면,
시스템 관리자가 자동으로 알림을 받습니다. 문제가 확인되면 도와드릴 수 있는지 연락드리겠습니다.

### 임포트 재실행

일부 마이그레이션에서는 임포트를 여러 번 실행해야 할 필요가 있습니다. 예를 들어, 테스트용으로 먼저 한 번 실행한 다음
전환하기 전에 최신 데이터로 임포트를 다시 실행하는 경우가 흔합니다.

동일한 콘텐츠를 재임포트해도 **중복이 생성되지 않습니다**.

### 데이터 보안 및 만료

임포트 파일은 외부 요청으로 어떤 방식으로도 접근할 수 없으며, 임포트가 완료되는 즉시 시스템에서 삭제됩니다.
---