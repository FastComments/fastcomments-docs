사용자가 댓글을 작성하거나 투표할 때 로그인하지 않은 상태라면 이메일과 사용자 이름을 입력하라는 안내가 표시됩니다.

익명 댓글 작성의 경우, 댓글 작성 시의 마찰을 줄이기 위해 기본 사용자 이름을 정의하는 것이 바람직할 때가 있습니다. 이는 Customization UI에서 설정할 수 있습니다. 먼저 익명 댓글 작성(Anonymous Commenting)이 활성화되어 있어야 합니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.allow-anonymous-comments']; selector = '.default-username-label'; alt='익명 댓글 작성이 활성화되면 Customization UI에 나타나는 기본 사용자 이름 필드'; title='기본 사용자 이름 설정' app-screenshot-end]

#### 기본 사용자 이름 공유

기본 사용자 이름은 공유되는 표시 이름이지, 신원(identity)이 아닙니다. 기본값을 그대로 사용하고 이메일을 입력한 방문자는 자신만의 계정을 갖게 되며, 기본 사용자 이름이 공개 이름으로 표시됩니다. 여러 방문자가 동일한 기본값을 사용할 수 있으므로 "Anonymous"가 이미 사용 중이라고 표시되는 일은 없습니다.

방문자가 직접 입력한 사용자 이름은 여전히 고유해야 합니다.

방문자가 직접 이름을 고민하지 않고도 각자 고유한 이름을 갖게 하고 싶다면 [사용자 이름 자동 생성](/guide-customizations-and-configuration.html#auto-generate-username) 문서를 참고하세요.