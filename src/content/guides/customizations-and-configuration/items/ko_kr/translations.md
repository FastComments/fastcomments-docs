---
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

FastComments를 사용하면 댓글 위젯의 모든 텍스트를 사용자 정의할 수 있습니다.

제출 버튼과 같은 단일 텍스트만 덮어쓸 수도 있고, 댓글 위젯 전체의 모든 텍스트를 덮어쓸 수도 있습니다.

기본적으로 댓글 위젯의 텍스트는 사용자의 로케일을 기준으로 번역됩니다. 그러나, 텍스트를 재정의할 수 있습니다, 만약 우리가 확신한다면
우리 사용자층이 동일한 로케일/언어를 사용하고 있다는 것을, 예를 들면:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

모든 사용자 정의 가능한 번역은 <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">여기</a>의 "고급 옵션" 탭에서 확인할 수 있습니다.

그러나 위젯 사용자 정의 UI를 통해 더 쉬운 방법이 있습니다. 거기에서 우리는 댓글 위젯에 EN_US 로케일로 표시되는 텍스트를 찾아 단순히
대체 텍스트를 지정할 수 있습니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

현재 모든 번역 재정의는 모든 로케일에 적용됩니다.

---