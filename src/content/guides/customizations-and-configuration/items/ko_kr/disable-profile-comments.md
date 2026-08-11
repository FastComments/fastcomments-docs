[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자 프로필에 "Profile Comments" 탭을 표시하여 방문자가 누군가의 프로필에 댓글을 남길 수 있게 합니다.

하지만 이 탭을 비활성화할 수 있습니다:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = '프로필 댓글 비활성화'; code-example-end]

코드 없이도 이 작업을 수행할 수 있습니다. 위젯 맞춤 설정 페이지에서 "프로필 댓글 비활성화" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; alt='프로필 댓글 탭을 숨기기 위해 프로필 댓글 비활성화 체크박스가 선택된 위젯 맞춤 설정 페이지'; title='프로필 댓글 비활성화' app-screenshot-end]