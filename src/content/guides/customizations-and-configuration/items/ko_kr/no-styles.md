[related-parameter-start name = 'noStyles'; type = 'boolean'; related-parameter-end]

보다 큰 맞춤 스타일링 프로젝트의 경우, 기본 스타일링을 전혀 사용하지 않고 깨끗한 상태에서 시작하는 것이 바람직할 수 있습니다.

다음과 같이 **noStyles** 매개변수를 true로 설정하면 모든 기본 스타일링을 제거할 수 있습니다:

[code-example-start config = {noStyles: true}; linesToHighlight = [6]; title = 'Disabling All Default Styles'; code-example-end]

코드 없이 위젯 맞춤 설정 페이지의 고급 옵션에서 이를 사용자 지정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.disable-all-default-styling']; selector = '.disable-all-default-styling'; alt='위젯 맞춤 설정 페이지의 고급 옵션에서 활성화된 모든 기본 스타일링 비활성화 체크박스'; title='모든 기본 스타일 비활성화' app-screenshot-end]