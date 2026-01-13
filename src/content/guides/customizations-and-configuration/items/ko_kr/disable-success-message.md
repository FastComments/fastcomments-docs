[related-parameter-start name = 'disableSuccessMessage'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글 작성 후 성공 메시지를 표시합니다. 다음과 같이 이 기능을 비활성화할 수 있습니다:

[code-example-start config = {disableSuccessMessage: true}; linesToHighlight = [6]; title = 'Disable Success Message'; code-example-end]

코드를 사용하지 않고도 이 작업을 수행할 수 있습니다. 위젯 사용자 지정 페이지에서:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-success-message']; selector = '.disable-success-message'; title='Disable Success Message' app-screenshot-end]