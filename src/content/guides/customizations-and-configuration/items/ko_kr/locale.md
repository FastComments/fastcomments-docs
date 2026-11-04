[related-parameter-start name = 'locale'; type = 'string'; related-parameter-end]

기본적으로 FastComments는 사용자의 시스템과 브라우저에 의해 결정된 로캘에 따라 댓글 위젯을 렌더링합니다.

사용자가 댓글을 작성하거나 로그인하면, 마지막으로 사용한 로캘을 업데이트하여 이메일 발송 등에도 이를 사용합니다.

이는 사용자에게 댓글 위젯이 번역되는 방식에 영향을 미칩니다. 로캘은 사용자의 언어와 지역으로 구성되므로, 로캘을 설정하면 일반적으로 사용자에게 표시되는 텍스트의 언어가 변경됩니다.

#### UI를 통해

위젯 사용자 지정 UI를 사용하여 이를 정의할 수 있습니다. "Locale / Language" 옵션을 참조하세요:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.locale-override'; alt='위젯 사용자 지정 페이지의 Locale / Language 드롭다운으로 방문자의 감지된 로캘을 재정의할 수 있습니다'; title='로캘 / 언어 변경' app-screenshot-end]

#### 코드를 통해

원하는 로캘로 재정의할 수 있습니다.

[code-example-start config = {locale: 'ru_ru'}; linesToHighlight = [6]; title = '사용자의 로캘 수동 정의'; code-example-end]

### 지원되는 언어 및 로캘 코드

[지원되는 언어 및 해당 로캘 코드의 전체 목록은 여기에서 확인할 수 있습니다.](/guide-supported-languages.html#supported-languages)

### SSO 참고 사항

SSO를 사용하는 경우, 이메일 및 기타 항목이 사용자에게 올바르게 현지화되도록 사용자 객체에 사용자의 로캘을 전달하는 것이 좋습니다.