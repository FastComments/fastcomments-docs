[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

기본적으로 FastComments 댓글 위젯은 `gif rating`을 `pg`로 설정합니다.

사용 가능한 옵션은 `g`, `pg`, `pg-13`, `r`입니다.

이 설정은 코드에서 또는 UI를 통해 지정할 수 있습니다. 코드에서는 다음과 같이 할 수 있습니다:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Gif 등급 설정'; code-example-end]

UI에서는 `Disable Image Uploads?` 옵션이 선택되지 않은 경우 `Gif Picker Rating` 아래에서 찾을 수 있습니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; alt='위젯 커스터마이징 페이지에서 g, pg, pg-13 및 r을 제공하는 Gif Picker Rating 드롭다운'; title='Gif 등급 설정' app-screenshot-end]