[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

기본적으로 FastComments는 사용자의 시스템 및 브라우저에서 결정된 로케일에 따라 댓글 위젯을 렌더링합니다.

사용자가 댓글을 작성하거나 로그인하면, 저희는 사용자가 마지막으로 사용한 로케일을 업데이트하고 이메일 전송 시에도 이를 사용합니다.

이는 댓글 위젯이 사용자에게 어떻게 번역되어 표시되는지에 영향을 줍니다. 로케일은 사용자의 언어와 지역으로 구성되므로, 로케일을 구성하면 일반적으로 사용자에게 표시되는 텍스트의 언어가 변경됩니다.

#### Via The UI

위젯 커스터마이징 UI에서 이 값을 정의할 수 있습니다. "Locale / Language" 옵션을 참조하세요:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; title='Changing The Locale / Language' app-screenshot-end]

#### Via Code

원하는 로케일로 이를 재정의할 수 있습니다.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### 지원되는 언어 및 로케일 코드

[지원되는 언어와 해당 로케일 코드의 전체 목록은 여기에서 확인할 수 있습니다.](/guide-supported-languages.html#supported-languages)

### SSO 참고

SSO를 사용하는 경우, 이메일 등 다른 항목들이 사용자에게 올바르게 현지화되도록 user 객체에 사용자의 locale을 전달하는 것이 좋습니다.