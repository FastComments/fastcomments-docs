FastComments는 첫 댓글 작성자가 댓글을 제출하기 전에 서비스 약관에 동의하도록 요구할 수 있습니다.

활성화하면:
- **익명 사용자**는 댓글을 작성할 때마다 TOS 체크박스가 표시됩니다
- **인증된 사용자**는 첫 댓글 작성 시 또는 TOS를 업데이트했을 때만 체크박스가 표시됩니다

### 설정

위젯 사용자 지정 페이지로 이동하여 "Require Terms of Service acceptance" 체크박스를 활성화하세요. 활성화하면 다음 옵션이 표시됩니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='TOS 텍스트 모드 선택기와 마지막 업데이트 날짜 필드가 표시된 서비스 약관 패널'; title='서비스 약관 옵션' app-screenshot-end]

- **TOS 텍스트 모드**: 기본적으로 체크박스에는 두 문서에 대한 링크와 함께 "I agree to the Terms of Service and Privacy Policy"가 표시됩니다. "Customize text per locale"을 선택하여 언어별로 사용자 지정 텍스트를 제공할 수 있습니다.
- **TOS 마지막 업데이트 날짜**: 서비스 약관을 업데이트할 때 이 날짜를 설정하세요. 이 날짜 이전에 동의한 사용자는 다시 동의해야 합니다.

### 작동 방식

- TOS 동의 타임스탬프는 사용자별 및 댓글별로 저장됩니다
- 사용자가 TOS에 동의하면 해당 날짜가 사용자 프로필(테넌트별)에 기록됩니다
- 사용자의 동의 날짜 이후로 "마지막 업데이트" 날짜를 설정하면 다시 동의해야 합니다
- 추적할 수 없는 익명 사용자의 경우 댓글을 제출할 때마다 체크박스가 표시됩니다