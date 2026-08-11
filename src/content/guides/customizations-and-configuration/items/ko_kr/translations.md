---
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments를 사용하면 댓글 위젯의 모든 텍스트를 사용자 정의할 수 있습니다.

제출 버튼과 같은 단일 텍스트 조각을 재정의하거나 전체 댓글 위젯의 모든 텍스트를 재정의할 수 있습니다.

기본적으로 댓글 위젯의 텍스트는 사용자의 로케일에 따라 번역됩니다. 그러나 사용자 기반이 동일한 로케일/언어를 사용하고 있다고 확신하는 경우 텍스트를 재정의할 수 있습니다. 예를 들어:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = '맞춤 텍스트'; code-example-end]

모든 사용자 정의 가능한 번역은 <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">여기</a>의 "고급 옵션" 탭에서 찾을 수 있습니다.

하지만 위젯 커스터마이징 UI를 통해 더 쉬운 방법이 있습니다. 여기서 EN_US 로케일의 댓글 위젯에 표시되는 텍스트를 찾아 교체 텍스트를 지정할 수 있습니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='드롭다운에서 선택된 위젯 문자열과 교체 텍스트 필드가 있는 맞춤 텍스트 패널'; title='맞춤 텍스트' app-screenshot-end]

현재 모든 번역 재정의는 모든 로케일에 영향을 미칩니다.

---