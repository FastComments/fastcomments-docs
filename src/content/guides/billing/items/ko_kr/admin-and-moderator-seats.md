Flex 플랜에서는 계정을 관리하거나 중재할 수 있는 사람이 월간 시트(seat)로 청구됩니다. 이것이 Flex 청구서에서 가장 많이 사람들을 놀라게 하는 부분인데, 대개 아무도 눈치채지 못한 사이에 시트가 생성되었기 때문입니다.

### 시트로 계산되는 항목

- **관리자(Admins)**: 계정의 `Users` 페이지에서 청구, 커스터마이징, 분석, 중재 등 어떤 관리 권한이든 가진 사용자입니다. 첫 번째 관리자(보통 계정 소유자)는 무료이며, 추가 관리자마다 시트 하나가 됩니다.
- **중재자(Moderators)**: `Moderators` 페이지에서 초대를 수락한 사람입니다. 대기 중인 초대는 청구되지 않습니다. 권한이 댓글 중재뿐인 사용자는 관리자가 아닌 중재자로 청구됩니다.
- **SSO 관리자** 및 **SSO 중재자**: 사이트에서 SSO 페이로드에 `isAccountOwner`, `isAdminAdmin`, 또는 `isCommentModeratorAdmin`이 설정된 상태로 로그인하는 SSO 사용자입니다. 일반 관리자 및 중재자와 동일한 요율로 청구됩니다. [SSO 사용자 구조](/guide-api.html#sso-user-structure)를 참고하세요.

시트는 이메일 주소 기준으로 중복 제거되므로, 관리자이면서 중재자인 사람, 또는 동일한 이메일로 SSO 사용자와 대시보드 로그인을 모두 가진 사람은 더 높은 요율로 한 번만 청구됩니다. 서로 다른 이메일 주소를 두 개 가진 사람은 시트 두 개입니다.

월간 활성 사용자(MAU) 기준으로 청구되는 계정에서는 해당 월에 로그인하지 않은 SSO 중재자는 그 달에 청구되지 않습니다.

### 직접 추가하지 않은 시트

두 가지 기능이 자동으로 시트를 생성합니다:

- **SSO 관리자 동기화.** SSO 사용자가 대시보드 사용자의 이메일과 일치하는 이메일로 로그인하면, 해당 사용자의 관리자 또는 중재자 권한이 SSO 사용자에게 복사됩니다. WordPress SSO의 경우 모든 WordPress 관리자가 FastComments 관리자 또는 중재자가 될 수 있다는 의미입니다. 커스터마이징 규칙의 `Customize` 아래 `Disable Automatic Admin Migration` 옵션으로 이 기능을 끌 수 있습니다.
- **같은 사람의 서로 다른 이메일.** 대시보드 로그인과 SSO 사용자가 서로 다른 이메일을 사용하면 시트 두 개가 됩니다. 양쪽에서 같은 이메일을 사용하면 하나로 병합됩니다.

### 시트 확인 및 제거

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 페이지에는 `Moderators`, `SSO Admins`, `SSO Moderators` 수가 표시되고, 다음 인보이스 미리보기에서 그 비용을 확인할 수 있습니다. 필요 없는 사람은 `Users` 및 `Moderators` 페이지에서 제거하거나, SSO 페이로드에서 관리자 플래그를 제거하세요. 시트는 월말에 인보이스가 생성될 때 집계되므로, 그 전에 제거된 시트는 해당 월에 청구되지 않습니다.

두 항목 모두 관리 방법은 [중재자 추가](/guide-users.html#adding-moderators) 및 [관리자 권한 업데이트](/guide-users.html#updating-administrator-permissions)를 참고하세요.