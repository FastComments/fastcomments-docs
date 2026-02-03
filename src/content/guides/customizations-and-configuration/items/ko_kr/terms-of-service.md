FastComments를 사용하면 처음 댓글을 다는 사용자에게 댓글을 제출하기 전에 귀하의 이용 약관(TOS)을 수락하도록 요구할 수 있습니다.

활성화되면:
- **익명 사용자**는 댓글을 작성할 때마다 이용 약관 확인란을 보게 됩니다
- **인증된 사용자**는 첫 댓글을 작성할 때 또는 귀하가 이용 약관을 업데이트할 때만 확인란을 보게 됩니다

### 이용 약관 활성화

위젯 사용자 지정 페이지로 이동하여 "Require Terms of Service acceptance" 확인란을 활성화하세요:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### 이용 약관 텍스트 사용자 지정

기본적으로 확인란에는 "I agree to the Terms of Service and Privacy Policy"라는 문구와 두 문서에 대한 링크가 표시됩니다. 필요에 따라 로케일별로 이 텍스트를 사용자 지정할 수 있습니다:

1. "Customize text per locale"를 선택하세요
2. 드롭다운에서 로케일을 선택하고 사용자 지정 텍스트를 입력하세요

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### 이용 약관 업데이트

이용 약관을 업데이트할 때 "Last Updated" 날짜를 설정하세요. 이 날짜 이전에 TOS에 동의한 사용자는 다시 동의해야 합니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### 작동 방식

- TOS 수락 타임스탬프는 사용자별 및 댓글별로 저장됩니다
- 사용자가 TOS에 동의하면 해당 날짜가 사용자 프로필(테넌트별)에 기록됩니다
- 사용자의 수락 날짜 이전의 "Last Updated" 날짜를 설정하면 사용자는 다시 동의해야 합니다
- 추적할 수 없는 익명 사용자의 경우 확인란이 모든 댓글 제출 시 표시됩니다