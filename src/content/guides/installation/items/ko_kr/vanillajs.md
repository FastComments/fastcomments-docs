The VanillaJS 버전의 위젯은 웹사이트에 댓글을 추가하는 가장 간단한 방법으로, 별도의 빌드 시스템이나 서버 측 코드를 요구하지 않습니다.

댓글을 사이트에 추가하려면 다음 코드 스니펫을 어떤 페이지에든 간단히 추가하세요:

[code-example-start config = {}; title = 'Simple Code Snippet'; code-example-end]

동일한 코드 스니펫을 여러 페이지에서 사용할 수 있으며, 각 페이지마다 자동으로 별도의 스레드를 생성합니다.

많은 애플리케이션에는 "HTML Embed Code" 옵션이 있습니다. 그것을 선택하고 위의 코드 스니펫을 붙여넣으세요.

*계정 없이도 사용해볼 수 있습니다!* 로그인하지 않은 경우 위의 스니펫에서 "tenantId: demo"가 보일 수 있습니다. 이렇게 하면
데모 계정을 사용하게 됩니다.

위젯 구성에 대한 문서는 <a href="/guide-customizations-and-configuration.html" target="_blank">여기</a>에서 확인할 수 있습니다.

All versions of the FastComments widget are wrappers around the core VanillaJS library. This allows us to add features and fix issues in one place - and the changes automatically propagate to the other variants of the commenting widget.