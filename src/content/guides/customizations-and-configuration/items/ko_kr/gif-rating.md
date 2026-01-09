[related-parameter-start name = 'gifRating'; type = 'string'; related-parameter-end]

기본적으로 FastComments 댓글 위젯은 `gif rating`을 `pg`로 설정합니다.

사용 가능한 옵션은 `g`, `pg`, `pg-13`, 및 `r`입니다.

이 설정은 코드에서 또는 UI를 통해 설정할 수 있습니다. 코드에서는 다음과 같이 할 수 있습니다:

[code-example-start config = {gifRating: 'pg-13'}; linesToHighlight = [6]; title = 'Set Gif Rating'; code-example-end]

UI에서는 `Disable Image Uploads?`가 체크되어 있지 않은 한 `Gif Picker Rating`에서 이것을 찾을 수 있습니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.gif-rating'; title='Setting The Gif Rating' app-screenshot-end]

---