---
`localhost`에 대해 프로덕션과 동일한 단계를 따르세요. 프로덕션 도메인 및 API 비밀키가 설정되어 있는지 확인하십시오.

먼저, [Webhooks admin](https://fastcomments.com/auth/my-account/manage-data/webhooks)으로 이동합니다. 이는 Manage Data -> Webhooks에서 접근할 수 있습니다.

구성 페이지는 다음과 같이 표시됩니다:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='도메인 선택기와 댓글 이벤트당 엔드포인트 URL 필드, 그리고 테스트 페이로드 전송이 포함된 Webhooks 관리 페이지'; title='Webhooks 구성'; cacheBuster = 'v3' app-screenshot-end]

이 페이지에서 각 댓글 이벤트 유형에 대한 엔드포인트를 지정할 수 있습니다.

각 이벤트 유형마다 Send Test Payload를 클릭하여 통합이 올바르게 설정되었는지 확인하십시오. 자세한 내용은 다음 섹션인 "Testing"을 참조하세요.

---