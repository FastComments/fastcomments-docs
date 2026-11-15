크레딧은 결제 통화로 FastComments 계정에 보유된 금액입니다. 크레딧은 카드가 청구되기 전에 구독 갱신 및 Flex 사용량을 포함한 향후 인보이스에 자동으로 적용됩니다.

크레딧은 선결제이며, 할인이나 포인트 제도가 아닙니다. `$50` 크레딧을 추가하면 다음 `$50` 인보이스가 카드 대신 해당 크레딧으로 지불됩니다. 잔액은 결제 처리기인 Stripe에 계정 크레딧으로 보유되며, 인보이스에서 사용할 때까지 계정에 유지됩니다.

크레딧 잔액을 유지하는 몇 가지 이유:

- 일정 기간을 미리 선결제합니다. 예를 들어 재무 부서가 매월 카드 결제보다 한 번의 큰 금액 결제를 선호하는 경우입니다.
- Flex 플랜에 버퍼를 유지하여 바쁜 달의 인보이스가 도래하기 전에 대비합니다.
- 카드로 한 번 결제한 후 갱신 시 잔액에서 자동으로 차감되도록 합니다.

### 크레딧 위치

[My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info)를 열고 `View Invoices` 및 `Invoice Settings` 옆에 있는 `Manage Credits`를 클릭합니다. 그러면 `Credits` 페이지가 열리며, 현재 `Credit balance`, `Add credit` 양식, 그리고 `Past top-ups`를 확인할 수 있습니다.

잔액이 있으면 `Billing Info` 페이지에도 `Credit balance` 항목이 표시되어 Credits 페이지를 열지 않고도 확인할 수 있습니다.

### 크레딧 추가 권한

`Account Owner` 또는 `Billing Admin` 권한이 있는 사용자만 크레딧을 추가할 수 있으며, 이는 플랜이나 결제 수단을 변경할 수 있는 사용자와 동일합니다. 다른 관리자에게는 `Manage Credits` 버튼이 표시되지 않습니다. [Who Can Manage Billing](#who-can-manage-billing)을 참조하세요.