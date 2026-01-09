[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

댓글 입력 필드에 입력할 수 있는 최대 문자 수는 **maxCommentCharacterLength** 매개변수로 제한할 수 있습니다.

기본값은 2000입니다.

이미지 URL과 같은 항목은 길이 계산에 포함되지 않습니다.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

이 설정은 위젯 맞춤 설정 페이지에서 코드 없이 사용자화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---