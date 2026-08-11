FastComments는 처음 댓글을 다는 사용자가 댓글을 제출하기 전에 서비스 약관에 동의하도록 요구할 수 있게 합니다.

활성화된 경우:
- **Anonymous users** will see a TOS checkbox every time they comment
- **Authenticated users** will see the checkbox only on their first comment, or when you update your TOS

### Configuration

Navigate to the widget customization page and enable the "Require Terms of Service acceptance" checkbox. Once enabled, you'll see the following options:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='서비스 약관 패널에 TOS 텍스트 모드 선택기와 마지막 업데이트 날짜 필드가 표시됩니다'; title='서비스 약관 옵션' app-screenshot-end]

- **TOS Text Mode**: 기본적으로 체크박스는 "I agree to the Terms of Service and Privacy Policy" 라는 문구와 두 문서에 대한 링크를 표시합니다. "Customize text per locale"을 선택하면 각 언어에 대한 자체 텍스트를 제공할 수 있습니다.
- **TOS Last Updated Date**: 서비스 약관을 업데이트할 때 이 날짜를 설정합니다. 이 날짜 이전에 동의한 사용자는 다시 동의해야 합니다.

### How It Works

- The TOS acceptance timestamp is stored per-user and per-comment
- When a user accepts the TOS, the date is recorded on their user profile (per-tenant)
- If you set a "Last Updated" date that is after the user's acceptance date, they will need to re-accept
- For anonymous users who cannot be tracked, the checkbox appears on every comment submission

---