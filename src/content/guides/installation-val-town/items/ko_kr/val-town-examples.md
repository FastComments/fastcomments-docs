이 가이드의 각 부분을 다루는 리믹스 가능한 공개 val 네 개입니다.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([라이브](https://fastcomments-blog.val.run))는 모든 게시물 아래에 댓글 스레드가 있고 인덱스에는 대량 댓글 수가 표시되는 Markdown 블로그입니다. 리믹스하는 즉시 작동하며, 환경 변수 하나만 설정하면 본인 계정으로 연결됩니다.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([라이브](https://fastcomments-sso.val.run))는 방문자를 Val Town 계정으로 로그인시키고 해당 신원을 위젯에 전달하여 두 번째 로그인이 필요 없게 합니다.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([라이브](https://fastcomments-webhooks.val.run))는 모든 전송에서 HMAC 서명을 검증하고 이벤트를 SQLite에 저장합니다. 테스트 페이로드에 서명하여 자기 자신에게 전송하는 버튼이 있어, 실제 웹훅을 설정하기 전에 검증이 성공하는 것을 확인할 수 있습니다.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([라이브](https://fastcomments-skills.val.run))는 위젯, SSO, REST API, 모더레이션, Disqus 마이그레이션을 다루는 FastComments 에이전트 스킬 라이브러리입니다. 리믹스하면 Val Town의 에이전트인 Townie가 `skills/` 디렉터리에서 스킬을 자동으로 가져오므로, 문서를 채팅에 붙여넣지 않아도 에이전트가 댓글 설정 방법을 알게 됩니다.

동일한 스킬은 `npx skills add fastcomments/skills`로 어디서든 설치할 수 있습니다.