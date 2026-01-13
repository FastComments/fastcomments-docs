FastComments에서는 `username`이 모든 사이트와 커뮤니티에서 유일해야 합니다. 이는 해당 값으로 fastcomments.com에 로그인하던 방식이었기 때문입니다.

The exception is SSO - since SSO users are tied to a specific FastComments.com Tenant - they can have any username within that community.

#### 표시 이름

FastComments는 `Display Name`도 지원합니다. 표시 이름은 반드시 유일할 필요가 없으며 이모지를 포함할 수 있습니다. [계정 세부정보](https://fastcomments.com/auth/my-account/edit-details) 페이지에서 표시 이름을 설정할 수 있습니다.

#### 표시 이름의 고유성

If two FastComments.com users, on two very different major communities both have similar names, then we'd like
them to just be able to set their preferred display name. However, we can't restrict where people comment with their global FastComments account, so sometimes this may
result in confusion.

다른 사용자나 중재자와 비슷한 이름을 가지는 것 자체만으로는 남용 사례가 아닙니다. 그러나 이를 악용하여 의도적으로 다른 사용자를 사칭하는 경우, 우리는 귀하의 계정에서 이 기능을 비활성화할 수 있으며, 귀하는 `username`만 사용할 수 있도록 제한될 수 있습니다.