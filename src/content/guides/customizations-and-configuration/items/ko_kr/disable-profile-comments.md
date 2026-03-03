[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자 프로필에 "Profile Comments" 탭을 표시하여 방문자가 다른 사람의 프로필에 댓글을 남길 수 있도록 합니다.

하지만 이 탭은 비활성화할 수 있습니다:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

이 작업은 코드 없이도 할 수 있습니다. 위젯 맞춤 설정 페이지에서 "Disable Profile Comments" 섹션을 참조하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; title='Disable Profile Comments' app-screenshot-end]