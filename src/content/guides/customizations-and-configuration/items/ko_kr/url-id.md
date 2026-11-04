[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

댓글 스레드를 렌더링하거나 댓글을 작성할 때, FastComments는 해당 댓글들이 어떤 페이지, 아티클, 또는 상품에 속하는지 알아야 합니다.

이를 위해 우리는 "URL ID"라고 부르는 것을 사용합니다. 이는 문자열이나 숫자 같은 식별자이거나 URL입니다.

기본적으로 urlId를 지정하지 않으면 페이지 URL이 사용됩니다. 현재 페이지 URL을 가져와서 일반적인 마케팅 파라미터나 추적 식별자를 제거하도록 정리합니다.

WordPress와 같은 서드파티 통합의 경우, 우리 플러그인은 일반적으로 현재 보고 있는 정보를 나타내는 식별자(예: 아티클/페이지 ID)를 URL ID로 사용합니다.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = '사용자 정의 URL ID 정의하기'; code-example-end]

이 문서에서 자주 언급할 것 중 하나는 <a href="https://fastcomments.com/auth/my-account/customize-widget/new">위젯 커스터마이제이션 UI</a>입니다.

이 UI를 사용하면 코드를 작성하지 않고도 댓글 위젯을 여러 방식으로 변경할 수 있습니다.

커스터마이제이션 규칙을 만들 때는 사이트의 모든 페이지에 적용되기를 원하는 경우가 많습니다. 하지만 특정 페이지의 댓글 위젯을 커스터마이즈하고 싶은 경우도 있습니다. 예를 들어 사용자 정의 스타일을 적용하거나, 해당 페이지의 댓글을 익명으로 만들 수 있습니다. 또한 어떤 페이지에서는 라이브 댓글이 바로 표시되고, 다른 페이지에서는 알림 버튼 아래에 숨겨지도록 설정할 수도 있습니다.

이 모든 것은 이 페이지의 URL ID 입력 필드를 통해 가능하며, 해당 필드는 다음과 같이 생겼습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='커스터마이제이션 규칙을 한 페이지 또는 */blog/* 같은 패턴에 적용하기 위해 사용되는 URL ID 필드'; title='위젯 커스터마이제이션 페이지의 URL ID 입력란' app-screenshot-end]

이 필드의 값은 댓글 위젯에 전달되는 *urlId* 파라미터와 일치해야 합니다. 커스터마이제이션 규칙이 *urlId*에 상관없이 적용되기를 원한다면, 이 필드를 비워 두거나 *를 입력하세요.

2023년부터 위젯 커스터마이제이션의 `URL ID` 필드는 이제 패턴도 허용합니다! 예를 들어 `*/blog/*`를 사용하여 블로그에 특화된 스타일을, `*/store/*`를 사용하여 스토어에 특화된 스타일을 적용할 수 있으며, 모두 동일한 도메인을 사용하면서 가능합니다.

### 주의 사항

1. 페이지에 해시 파라미터(예: example.com#page-1)가 있는 경우, 기본적으로 이것이 URL ID의 일부가 됩니다.
2. 마이그레이션 중, 예를 들어 WordPress에서 Gatsby로 이전할 때, 초기 마이그레이션 이후 URL ID 댓글 값을 마이그레이션해야 할 수 있습니다. 이 경우 저희에게 문의해 주세요.