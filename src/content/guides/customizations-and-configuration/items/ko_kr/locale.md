[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

기본적으로 FastComments는 사용자의 시스템 및 브라우저에서 결정된 로케일에 따라 댓글 위젯을 렌더링합니다.

사용자가 댓글을 달거나 로그인하면 마지막으로 사용한 로케일을 업데이트하고 이를 이메일 발송에도 사용합니다.

이는 사용자를 위한 댓글 위젯 번역 방식에 영향을 줍니다. 로케일은 사용자의 언어와 지역으로 구성되므로 로케일을 설정하면 일반적으로 사용자에게 표시되는 텍스트의 언어가 변경됩니다.

#### Via The UI

위젯 커스터마이징 UI를 사용하여 정의할 수 있습니다. "Locale / Language" 옵션을 확인하세요:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='위젯 커스터마이징 페이지에서 방문자의 감지된 로케일을 재정의하는 Locale / Language 드롭다운'; title='로케일 / 언어 변경' app-screenshot-end]

#### Via Code

원하는 로케일로 재정의할 수 있습니다.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = 'Manually Defining the User\'s Locale'; code-example-end]

### Supported Languages and Locale Codes

[지원되는 언어와 해당 로케일 코드를 전체 목록을 여기에서 확인하세요.](/guide-supported-languages.html#supported-languages)

### SSO Note

SSO를 사용하는 경우, 사용자 객체에 사용자의 로케일을 전달하여 이메일 및 기타 항목이 올바르게 현지화되도록 할 수 있습니다.