[related-parameter-start name = 'absoluteDates'; type = 'boolean'; related-parameter-end]

기본적으로 로컬화된 상대 날짜가 사용됩니다. 예를 들어, 최근에 남긴 댓글 옆에 "11분 전"이라고 표시될 수 있습니다.

절대 날짜를 사용해야 하거나 원하는 경우, 이 매개변수를 true로 설정합니다. 

[code-example-start config = {absoluteDates: true}; linesToHighlight = [6]; title = '절대 날짜 사용'; code-example-end]

코드를 사용하지 않고도 위젯 맞춤 설정 페이지의 고급 옵션에서 이를 사용자 지정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option', '.absolute-dates']; selector = '.absolute-dates'; alt='위젯 맞춤 설정 페이지의 고급 옵션에서 절대 날짜 토글이 켜진 상태'; title='절대 날짜 사용' app-screenshot-end]

---