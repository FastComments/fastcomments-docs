청구는 두 종류의 사용자로 제한됩니다:

- `Account Owner`, 즉 계정을 생성한 사용자.
- `Billing Admin` 권한이 있는 모든 사용자.

이 사용자들만 플랜을 변경하거나, 카드를 저장 또는 교체하거나, `Invoice Settings`를 열거나, Flex 지출 한도를 설정하고, 크레딧을 추가할 수 있습니다. 다른 사용자가 시도하면 `Only the account owner or a billing admin can change the plan or payment method.`라는 메시지가 표시됩니다.

청구 관련 이메일(예: 체험 기간 중간 알림, 결제 실패 경고, 비활성화 통지)은 `Billing Admin` 권한이 있는 사용자에게 발송됩니다. 예를 들어 재무 담당자처럼 해당 이메일을 받아야 하는 사람에게 권한을 부여하면, 대시보드에 대한 다른 접근 권한 없이도 이메일을 받을 수 있습니다. 체험 기간이 청구 정보 없이 종료된 후 발송되는 알림은 계정의 모든 대시보드 사용자에게 전송됩니다. 이러한 각 이메일에는 수신 거부 링크가 포함되어 있으며, 계정 알림을 거부한 사용자는 더 이상 수신하지 않습니다.

인보이스 자체는 [Invoice Settings](#invoice-settings)의 `Billing Email`로 전송되며, 이 이메일은 대시보드 사용자의 이메일일 필요가 전혀 없습니다.

[Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 페이지는 결제 세부 정보가 아닌 사용량을 표시하므로 대신 `Analytics Admin` 권한이 필요합니다.

권한을 부여하거나 제거하려면 `Users` 페이지를 열고 해당 사용자를 편집하세요. [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions)를 참조하세요. Flex의 경우 추가 관리자마다 청구되는 시트가 된다는 점에 유의하세요. [Admin and Moderator Seats](#admin-and-moderator-seats)를 참조하세요.

로그인한 상태에서 `Billing Info`가 전혀 보이지 않거나 홈 페이지로 돌려보내진다면, 대시보드 사용자가 아닌 댓글 작성자 계정으로 로그인한 것입니다. 로그아웃한 후 사이트를 소유한 계정으로 로그인하세요.