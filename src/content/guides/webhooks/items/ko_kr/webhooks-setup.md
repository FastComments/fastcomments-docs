프로덕션 환경에서 하는 것과 동일한 절차를 `localhost`에서도 따르세요. 프로덕션 도메인과 API Secrets이 설정되어 있는지 확인하세요.

먼저, [웹후크 관리자](https://fastcomments.com/auth/my-account/manage-data/webhooks)로 이동하세요. 이는 데이터 관리 -> 웹후크를 통해 접근할 수 있습니다.

구성 페이지는 다음과 같이 표시됩니다:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Webhooks Configuration' app-screenshot-end]

이 페이지에서 각 댓글 이벤트 유형에 대한 엔드포인트를 지정할 수 있습니다.

각 이벤트 유형마다 통합이 올바르게 설정되었는지 확인하려면 Send Test Payload 버튼을 꼭 클릭하세요. 자세한 내용은 다음 섹션 "Testing"을 참조하세요.