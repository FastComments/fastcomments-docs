---
기본적으로 FastComments는 댓글을 달기 위해 이메일을 요구합니다. 이메일이 유효할 필요는 없지만, 사용자가 전송된 링크를 클릭할 때까지 해당 댓글은 "Unverified Comment" 라벨이 표시됩니다.

하지만 이메일 요구 사항을 제거할 수 있습니다. 이메일 입력 필드는 여전히 표시되지만 더 이상 필수는 아닙니다.

이 설정은 위젯 사용자 정의 UI를 통해 구성할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.allow-anonymous-comments'; alt='위젯 사용자 정의 UI에서 익명 댓글 옵션으로, 이메일 필드를 선택 사항으로 만듭니다'; title='익명 댓글 활성화' app-screenshot-end]

사용자 이름은 여전히 필요합니다. 이 단계도 제거하려면, 모든 사용자가 공유하는 [기본 사용자 이름 설정](/guide-customizations-and-configuration.html#default-username) 를 설정하거나, FastComments가 각 방문자를 위해 [고유 사용자 이름 생성](/guide-customizations-and-configuration.html#auto-generate-username) 하도록 할 수 있습니다.
---