[related-parameter-start name = 'disableToolbar'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글을 작성할 때 텍스트 꾸미기 및 업로드
이미지를 위한 단축 버튼을 제공하기 위해 도구 모음을 표시합니다.

이 도구 모음은 코드에서 또는 맞춤 설정 UI를 통해 비활성화할 수 있습니다.

[code-example-start config = {disableToolbar: true}; linesToHighlight = [6]; title = 'Disabling The Toolbar'; code-example-end]

코드를 사용하지 않고도 이 작업을 수행할 수 있습니다. 위젯 맞춤 설정 페이지에서 "답글 도구 모음 비활성화" 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-toolbar']; selector = '.disable-toolbar'; title='Disabling The Toolbar' app-screenshot-end]

---