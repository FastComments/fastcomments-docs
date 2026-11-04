FastComments는 SSO(Single-Sign-On, 싱글 사인온)라고 불리는 기능을 통해 멤버십 전용 사이트와 연동됩니다. 회원님의 멤버들은 WordPress 사이트에 로그인하기만 하면, FastComments 계정을 만들거나 소셜 미디어로 로그인할 필요 없이 댓글을 작성할 수 있습니다. 회원님의 멤버들(관리자 포함)이 WordPress 사이트에 로그인되어 있다면 댓글을 작성할 수 있습니다. 또한 관리자와 중재자는 WordPress 블로그 게시물에서 직접 댓글을 중재할 수 있습니다.

<sup>(선택 사항)</sup> 사용자 경험을 개선하고 중재자의 통계 추적을 활성화하려면, 관리자를 [User & Administrators](https://fastcomments.com/auth/my-account/users)에, 중재자를 [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators)에 추가하는 것을 잊지 마세요.

플러그인 대시보드로 이동하여 "SSO Settings"를 클릭하면 SSO를 활성화할 수 있습니다.

먼저 사이트의 "Anyone can Register" 기능을 활성화해야 합니다.

모든 사용자 정보는 사용자가 댓글 스레드를 볼 때마다 [HMAC](https://en.wikipedia.org/wiki/HMAC)을 통해 안전하고 원활하게 FastComments로 전송됩니다.

멤버들을 FastComments 서버로 복사하기 위해 초기 또는 지속적인 동기화를 실행할 필요가 없습니다. 이는 블로그 게시물을 열어 댓글 스레드를 볼 때 자동으로 수행됩니다.

## 이름과 아바타

플러그인은 사용자가 댓글 스레드를 볼 때 FastComments 내 해당 사용자의 모든 댓글에 대해 표시 이름과 아바타를 자동으로 업데이트합니다. 플러그인이 `get_avatar_url`을 호출하기 때문에 Gravatar 또는 WordPress 내의 모든 아바타 관리 플러그인을 통해 아바타가 지원됩니다.