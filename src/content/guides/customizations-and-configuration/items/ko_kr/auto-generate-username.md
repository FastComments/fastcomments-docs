사용자가 로그인하지 않은 상태에서 댓글을 작성하거나 투표하면 이메일과 사용자 이름을 입력하라는 안내가 표시됩니다.

일부 사이트에서는 방문자에게 고유한 사용자 이름을 직접 만들도록 요구하는 것이, 특히 모바일에서는 진입 장벽이 될 수 있습니다. FastComments는 새 방문자마다 중립적인 사용자 이름을 생성하여 사용자 이름 필드에 미리 채워 넣을 수 있습니다. 예: `BraveOtter4172`.

방문자는 그대로 사용하거나 원하는 이름으로 바꿀 수 있습니다.

이 기능은 Customization UI에서 `Generate Usernames Automatically` 설정을 통해 활성화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='위젯 커스터마이즈 UI의 Generate Usernames Automatically 옵션'; title='Generate Usernames Automatically' app-screenshot-end]

#### 동작 방식

- 생성되는 각 이름은 고유합니다. 기존 계정과 대조하여 확인되며 해당 방문자의 브라우저 세션에 예약되므로, 두 방문자에게 동일한 이름이 제안되지 않습니다.
- 이름은 아직 이름이 없는 방문자에게만 생성됩니다. 로그인한 사용자, SSO 사용자, 이미 댓글을 작성한 방문자는 기존 이름을 유지합니다.
- [익명 댓글](/guide-customizations-and-configuration.html#allow-anon)이 켜져 있든 꺼져 있든 작동합니다. 익명 댓글이 꺼져 있으면 방문자는 여전히 이메일을 입력하지만, 사용자 이름을 고민할 필요는 없습니다.
- 이전에 사용한 적 있는 이메일을 입력한 재방문자는 기존 계정과 매칭되며, 해당 계정의 이름을 유지합니다.
- [기본 사용자 이름](/guide-customizations-and-configuration.html#default-username)도 설정되어 있는 경우, 생성된 이름이 우선 적용됩니다.