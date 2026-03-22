FastComments에서 사이트에 댓글을 달지 못하도록 사용자를 차단하는 방법은 두 가지가 있습니다.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">차단된 사용자</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

사용자를 차단할 때 차단 유형을 선택할 수 있으며, 영구 차단 또는 영구 섀도우 차단을 선택할 수 있습니다:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

사용자를 차단하는 두 번째 방법은 댓글 관리 페이지(Comment Moderation)의 각 댓글에 있는 차단 버튼을 클릭하는 것입니다.

차단 버튼을 클릭하면 차단 유형과 기간을 지정할 수 있는 몇 가지 옵션이 표시됩니다.

### 이메일 별칭

이메일로 사용자를 차단할 때 FastComments는 자동으로 `+` 별칭을 무시합니다. 예를 들어, `user+alias@gmail.com`을 차단하면
`user@gmail.com` 및 `user+other@gmail.com`과 같은 해당 주소의 다른 `+` 변형도 함께 차단됩니다.

### 섀도우 밴

섀도우 밴은 사용자의 댓글이나 투표가 실제로는 저장되지 않았음에도 불구하고 성공적으로 저장된 것처럼 보이게 만드는 차단 유형입니다.
특정 상황에서는 바람직할 수 있습니다.

### IP 주소로 차단

테넌트가 옵트아웃하지 않는 한, FastComments는 댓글 작성자의 IP 주소를 해시한 값을 저장하여 IP를 통한 차단을 지원합니다.