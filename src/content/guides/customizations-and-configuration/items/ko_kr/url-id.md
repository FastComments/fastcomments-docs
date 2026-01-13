[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

댓글 스레드를 렌더링 하거나 댓글을 남길 때, FastComments는 해당 댓글이 어느 페이지, 기사, 또는 상품에 속하는지 알아야 합니다.

이를 위해 우리는 "URL ID"라고 부르는 것을 사용합니다. 이는 문자열이나 숫자 같은 식별자이거나 또는 URL일 수 있습니다.

기본적으로 urlId를 지정하지 않으면 페이지 URL이 됩니다. 현재 페이지 URL을 가져와 흔히 사용되는 마케팅 파라미터나 추적 식별자는 제거하도록 정리합니다.

WordPress와 같은 서드파티 통합의 경우, 플러그인은 보통 현재 보고 있는 정보를 나타내는 식별자(예: 기사/페이지 id)를 URL ID로 사용합니다.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

이 문서에서 자주 참조하는 한 가지는 <a href="https://fastcomments.com/auth/my-account/customize-widget/new">위젯 사용자화 UI</a>입니다.

이 UI는 코드를 사용하지 않고도 댓글 위젯에 대한 다양한 변경을 적용하는 데 사용할 수 있습니다.

커스터마이즈 규칙을 만들 때, 일반적으로 사이트의 모든 페이지에 적용되도록 하려는 경우가 많습니다. 그러나 경우에 따라 특정 페이지에 대해 댓글 위젯을 맞춤 설정하고 싶을 수 있습니다. 예를 들어 특정 페이지에 대해 사용자 정의 스타일을 적용하거나 해당 페이지의 댓글을 익명으로 처리하도록 할 수 있습니다. 또한 일부 페이지에서는 라이브 댓글을 즉시 표시하고, 다른 페이지에서는 알림 버튼 아래에 숨길 수도 있습니다.

이는 이 페이지의 URL ID 입력 필드를 통해 모두 가능합니다. 해당 필드는 다음과 같이 보입니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

이 필드의 값은 댓글 위젯에 전달되는 *urlId* 매개변수와 일치해야 합니다. 커스터마이즈 규칙이 *urlId*에 구애받지 않게 하려면 이 필드를 비워두거나 *를 입력하세요.

As of 2023 the `URL ID` field in widget customization now also takes patterns! For example you may
have `*/blog/*` to add styling specific to your blog and `*/store/*` to have styling specific to your store,
all while using the same domain.

### 주의사항

1. 페이지에 해시 파라미터가 있는 경우(예: example.com#page-1) - 기본적으로 이것은 URL ID의 일부가 됩니다.
2. 마이그레이션 중(예: WordPress에서 Gatsby로) 초기 마이그레이션 이후 URL ID 댓글 값을 이전해야 할 수도 있습니다. 이 경우 저희에게 문의하세요.