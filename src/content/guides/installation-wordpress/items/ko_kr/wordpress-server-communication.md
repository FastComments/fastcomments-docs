플러그인이 작동하려면 토큰이 WordPress 데이터베이스와 FastComments 계정에 저장됩니다. 플러그인이 당사 서버에 요청을 보낼 때,
이 토큰을 제공합니다.

FastComments 계정에 승인된 모든 통합은 [여기](https://fastcomments.com/auth/my-account/manage-data/integrations)에서 확인할 수 있습니다.

모든 통신은 HTTPS를 통해 이루어집니다.

모든 통신은 WordPress 서버에서 FastComments.com으로 *아웃바운드* 방식으로 이루어집니다. WordPress 설치로의 동기화(*되돌아감*)도 구현되며
WordPress 설치의 [cron] 설정에서의 [polling]을 통해 이루어집니다.