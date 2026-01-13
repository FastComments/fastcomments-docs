---
기본적으로 FastComments는 사용자가 입력한 이름 또는 SSO를 통해 전달된 이름을 표시합니다.

그러나 사용자의 이름을 마스킹하거나 다르게 표시하는 것이 바람직할 수 있습니다. 예를 들어 사용자의 이름이 Allen Rex라면 "Allen R."만 표시하고 싶을 수 있습니다.

이것은 코드 없이 위젯 사용자화 UI에서 `Commenter Name Format`이라는 설정으로 수행할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

사용 가능한 형식은 다음과 같습니다:

- Capitalize (예시 사용자명을 Example User로 표시)
- Last Initial (예시 사용자명을 Example U.로 표시)
- All Initials (예시 사용자명을 E. U.로 표시)
- Show "Anonymous"

이 설정 변경의 효과는 즉시 반영됩니다. 사용자는 자신에게는 코멘트 영역 상단에 전체 사용자 이름이 그대로 보이지만, 그들이 올린 코멘트에는 수정된 사용자 이름이 표시됩니다.

사용자 이름은 사용자를 보호하기 위해 서버 측에서 마스킹됩니다.

---