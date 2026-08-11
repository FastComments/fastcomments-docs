[related-parameter-start name = 'disableToolbar'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글을 작성할 때 텍스트를 꾸미고 업로드  
이미지를 위한 바로 가기를 제공하는 툴바를 표시합니다.

이 툴바는 코드나 커스터마이제이션 UI를 통해 비활성화할 수 있습니다.

[code-example-start config = {disableToolbar: true}; linesToHighlight = [6]; title = 'Disabling The Toolbar'; code-example-end]

코드 없이도 이 작업을 수행할 수 있습니다. 위젯 커스터마이제이션 페이지에서 "Disable The Reply Toolbar" 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-toolbar']; selector = '.disable-toolbar'; alt='포맷팅 바로 가기를 제거하기 위해 Disable The Reply Toolbar 체크박스가 선택된 위젯 커스터마이제이션 페이지'; title='툴바 비활성화' app-screenshot-end]