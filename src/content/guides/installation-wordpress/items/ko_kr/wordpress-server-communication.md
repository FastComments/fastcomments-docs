---
플러그인이 작동하려면 토큰이 WordPress 데이터베이스와 FastComments 계정에 저장됩니다. 플러그인이 우리 서버에 요청을 보낼 때 이 토큰을 제공합니다.

FastComments 계정에 승인된 모든 통합을 [here](https://fastcomments.com/auth/my-account/manage-data/integrations)에서 볼 수 있습니다.

모든 통신은 HTTPS를 통해 이루어집니다.

모든 통신은 WordPress 서버에서 FastComments.com으로 *outbound*이며, WordPress 설치에 대한 동기화 *back*도 포함됩니다. 이는 WordPress 설치의 [cron](https://en.wikipedia.org/wiki/Polling_(computer_science) 설정에서 [polling](https://developer.wordpress.org/plugins/cron/))을 통해 구현됩니다.
---