FastComments에서 사이트에 댓글을 달지 못하도록 사용자를 차단하는 방법은 두 가지가 있습니다.

첫째, 이미 이메일을 알고 있다면 <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">차단된 사용자</a> 페이지에 입력할 수 있습니다.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

이 페이지는 댓글 중재 -> 차단된 사용자에서 접근할 수 있습니다

사용자를 차단할 때 유형을 선택할 수 있으며, '영구 차단' 또는 '영구 섀도우 차단' 중 하나를 고를 수 있습니다:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

두 번째 방법은 댓글 중재 페이지의 각 댓글에 있는 차단 버튼을 클릭하는 것입니다.

차단 버튼을 클릭하면 차단 유형과 기간을 지정할 수 있는 몇 가지 옵션이 표시됩니다.

### 섀도우 밴

섀도우 밴은 사용자의 댓글이나 투표가 실제로는 저장되지 않았음에도 불구하고 저장된 것처럼 보이게 하는 차단 유형입니다.
특정 상황에서는 이런 방식이 바람직할 수 있습니다.

### IP 주소로 차단

테넌트가 옵트아웃을 원하지 않는 한 FastComments는 댓글 작성자의 IP 주소를 해시한 버전을 저장하여 IP로 차단을 지원합니다.