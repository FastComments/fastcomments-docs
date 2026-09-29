---
FastComments 지원팀이 마이그레이션을 도와줄 수 있지만, 대부분은 지원 직원의 개입 없이도 쉽게 수행하고 모니터링할 수 있습니다.

다음 제공업체의 내보내기 데이터를 기본적으로 가져올 수 있습니다:

- Commento
- Disqus
- Hyvor Talk
- Muut Comments
- IntenseDebate
- Just-Comments
- Cusdis
- WordPress (via the plugin, or an XML or CSV export)
- AnyComment (Via WordPress Import/Export)

다음 링크 [here](https://fastcomments.com/auth/my-account/manage-data/import)를 통해 마이그레이션할 데이터를 포함한 파일을 업로드할 수 있습니다.

[app-screenshot-start url='/auth/my-account/manage-data/import'; selector = '.account-block'; alt='FastComments 가져오기 페이지로, 제공자 선택 및 내보내기 파일을 위한 파일 업로드 필드가 있습니다.'; title='가져오기 페이지 양식' app-screenshot-end]

### 가져오기 모니터링

FastComments는 가져오기 및 내보내기를 처리하기 위해 작업 처리 시스템을 사용합니다. 시스템이 작업을 수신하면, 가져오기 또는 내보내기 UI에서 작업 상태를 주기적으로 보고합니다.

[app-screenshot-start url='/auth/my-account/manage-data/import?demo=true'; selector = '.content'; alt='가져오기 페이지에 진행 중인 가져오기 작업과 작업 처리 시스템이 보고한 상태가 표시됩니다.'; title='가져오기 작업 상태' app-screenshot-end]

가져오기 및 내보내기 상태는 계정의 모든 관리자에게 표시됩니다.

작업이 실패하면 자동으로 재시작되지 않습니다. 가져오기를 다시 시도해야 합니다. 가져오기 또는 내보내기가 실패하면 시스템 관리자가 자동으로 알림을 받습니다. 문제가 확인되면 도움을 드릴 수 있는지 연락드리겠습니다.

### 가져오기 재실행

일부 마이그레이션에서는 가져오기를 여러 번 실행해야 할 수 있습니다. 예를 들어, 테스트를 위해 첫 번째 마이그레이션을 수행한 후, 스위치를 전환하기 전에 최신 데이터로 다시 가져오기를 실행하는 것이 일반적입니다.

같은 콘텐츠를 다시 가져와도 **중복이 생성되지 않습니다**.

### 데이터 보안 및 만료

가져오기 파일은 외부 요청을 통해 어떠한 방식으로도 접근할 수 없으며, 가져오기가 완료되는 즉시 시스템에서 삭제됩니다.

---