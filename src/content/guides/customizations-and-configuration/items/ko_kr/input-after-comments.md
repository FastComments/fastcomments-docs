[related-parameter-start name = 'inputAfterComments'; type = 'boolean'; related-parameter-end]

기본적으로 댓글 입력 영역은 댓글 스레드의 **앞에** 있습니다. 그러나 이 구성 매개변수를
true로 설정하면 이를 **뒤로** 이동할 수 있습니다.

[code-example-start config = {inputAfterComments: true}; linesToHighlight = [6]; title = 'Moving The Reply Box to The Bottom'; code-example-end]

이것은 코드 없이 위젯 사용자 지정 페이지에서 맞춤 설정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.input-after-comments'; title='Moving The Reply Box to The Bottom' app-screenshot-end]