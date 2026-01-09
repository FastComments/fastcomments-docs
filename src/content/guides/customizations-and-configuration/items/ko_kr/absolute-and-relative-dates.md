[related-parameter-start name = 'absoluteAndRelativeDates'; type = 'boolean'; related-parameter-end]

기본적으로 지역화된 상대 날짜가 사용됩니다. 예를 들어, 최근에 남긴 댓글 옆에 "11 minutes ago"가 표시될 수 있습니다.

이 상대 날짜 형식을 유지하면서 전체 날짜도 함께 표시해야 할 경우가 있을 수 있으며, 이 경우 이 매개변수를 true로 설정합니다. 

[code-example-start config = {absoluteAndRelativeDates: true}; linesToHighlight = [6]; title = 'Use Both Absolute and Relative Dates'; code-example-end]

코드 없이도 위젯 사용자화 페이지의 고급 옵션에서 이 설정을 사용자화할 수 있습니다. UI에서 이 옵션을 보려면 먼저 절대 날짜(Absolute Dates)를 활성화해야 합니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates', '.relative-and-absolute-dates']; selector = '.relative-and-absolute-dates'; title='Use Both Absolute and Relative Dates' app-screenshot-end]

---