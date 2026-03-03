[related-parameter-start name = 'simpleSSO'; type = 'FastCommentsSSOSimple'; typeLink = 'https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts#L14' related-parameter-end]

Simple SSO를 사용하면 댓글 위젯에 사용자의 정보를 제공하여 사용자가 댓글을 달 때 사용자 이름이나 이메일을 입력할 필요가 없게 할 수 있습니다.

Simple SSO는 다음과 같이 구성할 수 있습니다:

[code-example-start config = {simpleSSO: { username: "Bob", email: "bob@example.com", avatar: "https://example.com/bob.png", websiteUrl: "https://example.com/profiles/bob", displayName: "Bob's Name", displayLabel: "VIP User", loginURL: 'https://example.com/login', logoutURL: 'https://example.com/logout', badgeConfig: { badgeIds: ['badge-id-1', 'badge-id-2'], pageBadgeIds: ['badge-id-3'], override: false } }}; linesToHighlight = [6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]; title = 'Simple SSO'; code-example-end]

사용자는 로그인된 상태가 되며, 내부적으로 SSO 사용자가 생성됩니다. API에서 가져온 경우 사용자의 `createdFromSimpleSSO`는 `true`로 설정됩니다.

Notes: 

- 이메일은 Simple SSO의 고유 식별자입니다.
- Simple SSO에 이메일을 제공하는 것은 필수가 아니지만, 기본적으로 이들의 댓글은 "확인되지 않음"으로 표시됩니다. <b>이메일이 제공되지 않으면 사용자는 완전히 인증될 수 없습니다.</b>
- **NEW** 2022년 1월 이후: 사용자 이름은 fastcomments.com 전체에서 고유할 필요가 없습니다
- 이메일이 제공되고 사용자가 원래 Secure SSO에서 생성되지 않은 경우 Simple SSO는 SSO 사용자를 자동으로 생성하고 업데이트할 수 있습니다.
- 사용자에 대한 배지는 `badgeConfig` 속성으로 지정할 수 있습니다. `badgeIds` 배열에는 사용자와 연결할 글로벌 배지의 ID들이 포함됩니다. `pageBadgeIds` 배열에는 현재 페이지(`urlId`)에 범위가 지정된 배지 ID들이 포함되며 — 이러한 배지는 할당된 페이지에서만 표시됩니다. `override`가 `true`로 설정되면 기존에 표시된 배지들을 대체합니다(글로벌과 페이지 범위 배지는 각각 독립적으로 대체됩니다); `false`이면 기존 배지에 추가됩니다.

---