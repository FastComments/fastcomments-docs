---
Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) 은 각 게시물 아래에 스레드가 있고 인덱스에 전체 댓글 수가 표시되는 Markdown 블로그입니다. 리믹스하는 순간 바로 작동하며, 하나의 환경 변수가 여러분의 계정으로 연결됩니다.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) 은 방문자를 Val Town 계정으로 로그인시키고 그 신원을 위젯에 전달하므로 두 번째 로그인이 필요하지 않습니다.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) 은 모든 전달에 대해 HMAC 서명을 검증하고 이벤트를 SQLite에 저장합니다. 테스트 페이로드에 서명하고 자체에 전달하는 버튼이 있어 실제 웹훅을 설정하기 전에 검증이 성공하는 것을 확인할 수 있습니다.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) 은 위젯, SSO, REST API, 모더레이션, Disqus에서 마이그레이션까지를 다루는 FastComments 에이전트 스킬 라이브러리입니다. 이를 리믹스하면 Val Town의 에이전트인 Townie가 `skills/` 폴더에서 자동으로 스킬을 가져와, 채팅에 문서를 붙여넣지 않아도 에이전트가 댓글을 설정하는 방법을 알게 됩니다.

The same skills install anywhere else with `npx skills add fastcomments/skills`.

---