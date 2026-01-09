[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

기본적으로 로컬화된 상대 날짜가 사용됩니다. 예를 들어, 방금 남긴 댓글 옆에는 "11분 전"과 같은 문구가 표시될 수 있습니다.

절대 날짜를 사용해야 하거나 사용하려는 경우가 있을 수 있으며, 이 경우 이 매개변수를 true로 설정합니다. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = 'Use Absolute Dates'; code-example-end]

이 설정은 위젯 사용자화 페이지의 고급 옵션에서 코드 없이 사용자화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; title='Use Absolute Dates' app-screenshot-end]