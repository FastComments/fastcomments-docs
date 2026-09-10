---
`localhost`에 대해 프로덕션과 동일한 단계를 따르세요. 프로덕션 도메인 및 API 비밀키가 설정되어 있는지 확인하세요.

먼저, [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks)으로 이동합니다. 이는 Manage Data -> Webhooks에서 접근할 수 있습니다.

이 페이지는 계정에 있는 모든 웹훅을 나열합니다:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='각 웹훅의 URL, 이벤트, 도메인, 메서드, 상태 및 대기 중인 이벤트 수를 나열하는 Webhooks 관리 페이지'; title='Webhooks 목록'; cacheBuster = 'v4' app-screenshot-end]

**New Webhook**을 클릭하여 추가합니다. 각 웹훅은 URL, 하나의 댓글 이벤트(생성, 업데이트 또는 삭제), 도메인 및 HTTP 메서드를 가집니다:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks/new'; selector = '.content'; alt='URL, 이벤트, 도메인 및 HTTP 메서드 필드와 Send Test Payload가 포함된 새 웹훅 양식'; title='새 웹훅'; cacheBuster = 'v4' app-screenshot-end]

각 웹훅은 독립적으로 전달됩니다. 동일한 이벤트를 여러 엔드포인트에 보낼 수 있으며, **All Domains**에 범위가 지정된 웹훅은 동일한 이벤트에 대한 도메인별 웹훅이 존재하더라도 모든 도메인의 댓글을 수신합니다. 동일한 URL, 이벤트 및 도메인은 두 번 추가할 수 없습니다.

저장하기 전에 **Send Test Payload**을 클릭하여 엔드포인트가 서명된 요청을 수락하는지 확인합니다. 자세한 내용은 다음 섹션인 "Testing"을 참조하세요.

목록에서 웹훅을 편집, 비활성화, 재활성화 또는 삭제할 수 있습니다. 비활성화하면 웹훅이 재활성화될 때까지 대기 중인 이벤트가 유지되며, 삭제하면 이벤트가 폐기됩니다.

웹훅은 Zapier와 같은 API를 통해서도 생성할 수 있습니다. 이러한 웹훅은 **API** 소스로 동일한 목록에 표시됩니다. API를 통한 웹훅 관리를 참조하세요.

---