FastComments를 사용하여 사이트에서 사용자가 댓글을 달지 못하도록 차단하는 방법은 두 가지가 있습니다.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">차단된 사용자</a> 페이지.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### 섀도우 밴

섀도우 밴은 사용자의 댓글이나 투표가 성공적으로 저장된 것처럼 보이게 하지만 실제로는 저장되지 않도록 하는 유형의 차단입니다. 특정 상황에서는 이것이 바람직할 수 있습니다.

### IP 주소를 통한 차단

테넌트가 옵트아웃을 원하지 않는 한, FastComments는 댓글 작성자의 IP 주소를 해시한 값을 저장하여 IP를 통한 차단을 지원합니다.