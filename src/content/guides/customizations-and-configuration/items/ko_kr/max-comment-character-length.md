---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

댓글 입력 필드에 입력할 수 있는 최대 문자 수는 **maxCommentCharacterLength** 매개변수로 제한할 수 있습니다.

기본값은 2000입니다.

이미지 URL과 같은 항목은 길이 계산에 포함되지 않습니다.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = '댓글 길이 제한'; code-example-end]

코드를 사용하지 않고 위젯 커스터마이징 페이지에서 이 설정을 맞춤화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='위젯 커스터마이징 페이지의 최대 댓글 크기 필드로, 댓글에 포함될 수 있는 문자 수를 제한하는 데 사용됩니다.'; title='댓글 길이 제한' app-screenshot-end]

---