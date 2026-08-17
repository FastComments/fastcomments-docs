By default, each user can submit up to `5 comments` in the same minute.

This is tracked by user id, anon user id, and ip address (hashed).

This can be customized without code, on the widget customization page:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='위젯 커스터마이징 페이지의 분당 최대 댓글 수 필드, 기본값은 5로 설정됨'; title='사용자당 댓글 양 제한' app-screenshot-end]

Note that if you're using the comment creation API may want to pass the user's original `ip` address in the request to our backend so rate limiting is applied
per user and not globally to your account.