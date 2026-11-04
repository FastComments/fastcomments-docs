[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO를 사용하면, 사용자가 댓글을 달기 위해 사용자 이름이나 이메일을 입력할 필요 없이 댓글 위젯에 사용자 정보를 제공할 수 있습니다.

Simple SSO는 다음과 같이 구성할 수 있습니다:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

사용자는 로그인된 상태가 되며, 백그라운드에서 SSO 사용자가 생성됩니다. API에서 가져온 경우 사용자는 `createdFromSimpleSSO`가 `true`로 설정됩니다.

참고: 

- 이메일은 Simple SSO의 고유 식별자입니다.
- Simple SSO에 이메일을 제공하는 것은 필수가 아니지만, 기본적으로 해당 사용자의 댓글은 "Unverified"로 표시됩니다. <b>이메일이 제공되지 않으면 사용자를 완전히 인증할 수 없습니다.</b>
- **NEW** 2022년 1월 이후: 사용자 이름은 fastcomments.com 전체에서 고유할 필요가 없습니다.
- Simple SSO는 이메일이 제공되고 사용자가 원래 Secure SSO에서 생성되지 않은 경우 SSO 사용자를 자동으로 생성 및 업데이트할 수 있습니다. Simple SSO 페이로드가 Secure SSO 또는 API에 의해 생성된 사용자를 지정하면 해당 사용자는 그대로 유지되고 방문자는 별도의 사용자를 얻게 됩니다.
- 사용자에게 배지를 지정하려면 `badgeConfig` 속성을 사용할 수 있습니다. `badgeIds` 배열은 사용자와 연결할 전역 배지 ID를 포함합니다. `pageBadgeIds` 배열은 현재 페이지(`urlId`)에 한정된 배지 ID를 포함하며, 이러한 배지는 할당된 페이지에서만 표시됩니다. `override`가 `true`로 설정되면 기존에 표시된 배지를 교체합니다(전역 및 페이지 범위 배지는 독립적으로 교체됨). `false`인 경우 기존 배지에 추가됩니다.

#### Simple SSO 설정

Simple SSO는 항상 사용할 수 있습니다. 두 가지 설정이 FastComments가 이를 얼마나 신뢰할지 제어합니다. <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">커스터마이징 규칙</a>에서 SSO를 검색하면 찾을 수 있습니다. 두 설정 모두 기본적으로 비활성화되어 있으며, "Require SSO To View Comments"가 켜져 있든 꺼져 있든 적용됩니다.

이 설정이 추가될 때 이미 Simple SSO를 사용하고 있던 계정은 두 설정이 모두 켜져 있었으므로 해당 계정에는 변화가 없습니다.

페이지와 가장 잘 일치하는 커스터마이징 규칙만 적용되므로, 규칙이 여러 개 있는 경우 각각에 이 설정을 적용해야 합니다. 다른 규칙 중 하나에 이 설정이 켜져 있으면 새 규칙도 기본적으로 켜진 상태로 시작합니다.

##### Simple SSO를 통한 자동 댓글 인증

이 옵션이 켜져 있으면 Simple SSO 사용자의 댓글이 즉시 인증됩니다.

이 옵션이 꺼져 있으면 Simple SSO 사용자의 댓글은 Simple SSO 페이로드에 있는 주소로 이메일을 보내고 사용자가 해당 링크를 클릭할 때까지 "Unverified"로 표시됩니다. 인증된 댓글만 자동 승인하도록 설정한 경우에도 해당 링크를 기다리게 됩니다. 페이로드에 이메일이 없으면 댓글은 계속 미확인 상태로 유지됩니다.

이 설정은 댓글에만 영향을 미칩니다. Simple SSO 사용자의 투표는 항상 즉시 집계됩니다.

##### Simple SSO 관리자 허용

이 옵션이 켜져 있으면 Simple SSO 사용자가 계정의 관리자 또는 중재자가 될 수 있습니다:

- 관리자 또는 중재자 권한을 가진 Simple SSO 사용자는 해당 권한을 유지합니다.
- 관리자 또는 중재자 중 하나와 동일한 이메일을 가진 Simple SSO 사용자는 해당 권한을 획득합니다.

이 옵션이 꺼져 있으면 Simple SSO는 이러한 권한을 부여하지 않습니다. 해당 권한을 가진 사용자를 지정하는 Simple SSO 페이로드는 권한이 없는 별도의 사용자를 생성하며, 원래 사용자는 그대로 유지됩니다.

Simple SSO 페이로드는 서명되지 않으므로, 이 옵션이 켜져 있으면 관리자의 이메일이나 사용자 ID를 아는 누구든지 해당 관리자로 가장할 수 있습니다. 사이트에서 이것이 문제가 된다면 이 옵션을 끄고 관리자는 Secure SSO를 사용하십시오.

Secure SSO 또는 API에 의해 생성된 사용자는 Simple SSO가 접근할 수 없으며, 이 옵션이 켜져 있든 꺼져 있든 마찬가지입니다.