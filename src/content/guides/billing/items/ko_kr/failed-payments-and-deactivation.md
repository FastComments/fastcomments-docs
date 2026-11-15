카드 만료, 카드 교체, 잔액 부족, 또는 은행의 결제 차단 등으로 인해 결제가 실패할 수 있습니다. FastComments는 문제가 발생하기 전에 이를 해결할 시간을 주고, 해결할 수 있는 사람들에게 알려줍니다.

### 어떤 일이 발생하나요

1. **결제가 실패합니다.** 인voice는 [Billing History](https://fastcomments.com/auth/my-account/billing-history)에서 `Unpaid` 상태로 유지되며 `Pay Now` 버튼이 표시됩니다. 고정 요금제에서는 Stripe가 결제를 재시도할 수 있습니다. Flex에서는 FastComments가 유예 기간이 지난 미결제 invoice를 매일 밤 재시도합니다.
2. **경고가 발송됩니다.** 매일 밤 FastComments는 모든 계정을 확인합니다. Flex invoice가 유예 기간이 지나도 미결제 상태이고 재시도가 실패하거나, 고정 요금제 구독이 결제 실패로 종료된 경우, `Billing Admin` 권한이 있는 모든 사용자에게 날짜와 함께 `Your FastComments Tenant Will Be Deactivated Soon!` 이메일이 발송되며, 한 유예 기간 후 계정 비활성화가 예약됩니다. 그동안 위젯은 계속 작동합니다.
3. **비활성화.** 예약된 날짜까지 아무 변화가 없으면 계정이 비활성화됩니다: 사이트에서 댓글이 숨겨지고 방문자에게는 `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` 메시지가 표시됩니다. Billing Admin에게 다시 이메일이 발송됩니다. 데이터는 보관됩니다.

유예 기간은 대부분의 계정에 대해 1주일이고, 1년 이상 된 계정에는 2주일입니다. Flex에서는 두 번 적용됩니다. 경고 전에 한 번, 경고와 비활성화 사이에 한 번이므로, 결제 실패부터 댓글이 숨겨지기까지 최소 2주가 걸립니다.

### 해결 방법

[Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지에서 사용 가능한 카드를 저장하세요. 이것만으로도 계정이 즉시 다시 활성화되고 예약된 비활성화가 취소됩니다. 그 후 미결제 invoice는 새 카드로 청구되며, 고정 요금제에서는 즉시, Flex에서는 매일 밤 확인 중에 처리됩니다. Billing History에서 `Pay Now`로 invoice를 즉시 결제할 수도 있습니다.

등록된 카드에 문제가 없고 결제 실패가 일시적이었다면 아무것도 할 필요가 없습니다: 매일 밤 확인 중에 결제된 invoice 또는 활성 구독을 감지하여 비활성화를 취소하고, Billing Admin에게 확인 이메일을 보냅니다. 이미 비활성화된 계정도 같은 방식으로 다시 활성화됩니다.

### 여러 invoice가 한 번에 청구되는 경우

카드가 몇 달 동안 유효하지 않았다가 수정된 경우, 미결제된 각 월이 개별적으로 청구되므로 같은 날에 여러 건의 결제가 발생할 수 있습니다. Billing History에서 각 invoice의 청구 기간을 확인하세요: 서로 다른 월에 대한 것이지 중복이 아닙니다.

### 결제 후에도 결제 경고가 표시되나요?

경고는 특정 계정에 속합니다. 결제했는데도 사이트에 `billing information is out of date`가 계속 표시된다면, 해당 사이트의 위젯이 결제한 계정이 아닌 다른 계정을 사용하고 있을 가능성이 높습니다. 이는 사이트가 두 번 설정되었거나, `fastcomments.com`의 계정과 EU 배포판인 `eu.fastcomments.com`의 계정이 각각 별도로 청구되는 경우에 발생합니다. 페이지 코드의 `tenantId`를 `Settings -> API/SSO` 아래의 값과 비교하거나, [지원팀에 문의](https://fastcomments.com/auth/my-account/help)하시면 해결해 드리겠습니다.