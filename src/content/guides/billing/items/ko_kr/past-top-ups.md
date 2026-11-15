크레딧 페이지 하단의 `Past top-ups` 테이블에는 가장 최근의 충전 내역이 최신 항목부터 표시되며, 각 항목의 `Date`, `Amount`, `Status`가 함께 나타납니다. 첫 충전 전에는 `No top-ups yet.`이라고 표시됩니다.

각 충전 항목은 세 가지 상태 중 하나를 가집니다:

- `Credited`: 결제가 성공했으며 금액이 잔액에 추가되었습니다.
- `Pending`: 결제가 시작되었지만 아직 완료되지 않았으며, 일반적으로 은행 확인이 완료되지 않은 경우입니다. 대기 중인 충전 항목은 크레딧 페이지를 다음에 열 때 Stripe에서 확인되어 `Credited` 또는 `Failed`로 변경됩니다.
- `Failed`: 카드가 거절되었거나 은행이 결제를 승인하지 않았습니다. 청구된 금액이 없으며 잔액은 변경되지 않았습니다.

이 테이블은 이 페이지에서 이루어진 충전 내역을 기록합니다. 크레딧으로 결제된 인보이스는 인보이스 자체와 함께 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지의 `View Invoices` 아래에 표시됩니다.