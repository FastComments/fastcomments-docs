[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

더 큰 맞춤 스타일링 프로젝트에서는 깨끗한 상태에서 시작하여 기본 스타일을 전혀 사용하지 않는 것이 바람직할 수 있습니다.

모든 기본 스타일은 다음과 같이 **noStyles** 매개변수를 true로 설정하여 제거할 수 있습니다:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

이 설정은 코드 없이도 위젯 커스터마이즈 페이지의 고급 옵션(Advanced Options)에서 변경할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; title='Disabling All Default Styles' app-screenshot-end]