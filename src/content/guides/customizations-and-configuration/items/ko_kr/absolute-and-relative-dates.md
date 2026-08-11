[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

기본적으로 로컬화된 상대 날짜가 사용됩니다. 예를 들어, 최근에 남긴 댓글 옆에 "11분 전"과 같은 표시가 보일 수 있습니다.

이 상대 날짜 형식을 유지하면서 전체 날짜를 함께 표시해야 하거나 원하는 경우, 이 매개변수를 true로 설정하면 됩니다. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

이는 코드를 사용하지 않고 위젯 사용자 정의 페이지의 고급 옵션에서 맞춤 설정할 수 있습니다. UI에서 이 옵션을 보려면 먼저 절대 날짜를 활성화해야 합니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; alt='위젯 사용자 정의 페이지의 고급 옵션에서 절대 날짜와 결합된 상대 날짜 설정이 모두 활성화된 상태'; title='절대 날짜와 상대 날짜 모두 사용' app-screenshot-end]