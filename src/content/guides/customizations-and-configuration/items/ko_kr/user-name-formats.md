---
기본적으로 FastComments는 사용자가 입력한 이름이나 SSO를 통해 전달된 이름을 표시합니다.

하지만 사용자의 이름을 가리거나 다른 방식으로 표시하고 싶을 수도 있습니다. 예를 들어, 사용자의 이름이 Allen Rex인 경우 "Allen R."만 표시하고 싶을 수 있습니다.

이것은 `Commenter Name Format`이라는 설정 아래 위젯 커스터마이제이션 UI에서 코드를 작성하지 않고도 할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format 드롭다운이 Capitalize, Last Initial 및 All Initials와 같은 선택지를 열어 보여줍니다'; title='이름 형식 변경' app-screenshot-end]

사용 가능한 형식은 다음과 같습니다:

- Capitalize (예시 사용자를 Example User로 표시)
- Last Initial (예시 사용자를 Example U.로 표시)
- All Initials (예시 사용자를 E. U.로 표시)
- "Anonymous" 표시

이 설정을 변경하면 효과가 즉시 적용됩니다. 사용자는 댓글 영역 상단에서 자신의 전체 사용자 이름을 볼 수 있지만, 댓글에는 수정된 사용자 이름이 표시됩니다.

사용자 이름은 서버 측에서 마스킹되어 사용자를 보호합니다.
---