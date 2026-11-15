충전 금액은 계정에 저장된 결제 수단으로 청구됩니다. 아직 카드를 저장하지 않았다면 Credits 페이지에 `Add a payment method first` 메시지와 `Go to billing` 버튼이 표시됩니다. [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지에서 카드를 저장한 후 다시 돌아오세요.

크레딧을 추가하려면:

1. [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info)를 열고 `Manage Credits`를 클릭합니다.
2. `Add credit`에서 미리 설정된 금액 중 하나를 선택하거나, `Other amount`를 선택하여 직접 금액을 입력합니다.
3. `Add` 버튼을 클릭합니다. 버튼에는 선택한 금액이 표시됩니다(예: `Add $25.00`).
4. 은행에서 결제 확인을 요구하는 경우, 열리는 창에서 확인 절차를 완료합니다.

결제가 완료되면 페이지에 `Credit added` 메시지와 함께 새 잔액이 표시되고, `Past top-ups`에 해당 충전 내역이 `Credited`로 나타납니다. 다시 충전하려면 `Add more`를 클릭하세요.

### 금액 및 한도

미리 설정된 금액은 청구 통화 기준으로 `10`, `25`, `50`, `100`입니다. 사용자 지정 금액은 충전당 `5`에서 `10,000` 사이의 금액이 가능하며, 사용 중인 통화의 정확한 한도는 폼 아래에 표시됩니다. 일본 엔화처럼 소수점 자릿수가 없는 통화의 경우, 미리 설정된 금액과 한도는 100배로 표시되어 대략적으로 같은 금액을 나타냅니다.

충전은 항상 계정의 청구 통화, 즉 인보이스와 동일한 통화로 진행됩니다. 어떤 통화로 청구되는지는 페이지에서 확인할 수 있습니다.

### 은행 확인(3D Secure)

일부 은행은 온라인 결제 확인을 요구하며, 보통 은행 앱이나 일회용 코드를 통해 진행됩니다. 해당하는 경우, `Add`를 클릭한 후 확인 창이 열립니다. 카드 청구와 잔액 업데이트는 은행이 승인한 후에만 이루어집니다.

확인을 완료하기 전에 페이지를 닫으면 해당 충전은 `Pending`으로 기록됩니다. 다음에 Credits 페이지를 열 때 Stripe를 통해 결제 상태를 확인하여 `Credited` 또는 `Failed`로 표시합니다. 은행이 결제를 승인하지 않으면 페이지에 `Your bank did not approve the payment. Try again, or use a different card.` 메시지가 표시되며, 청구되는 금액은 없습니다.

### 카드 거절

카드가 거절되면 페이지에 `Your card was declined. Check your card on the billing page, or try another card.` 메시지가 표시됩니다. 청구되는 금액은 없고 잔액도 변하지 않으며, 해당 시도는 `Past top-ups`에 `Failed`로 기록됩니다. [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지에서 카드를 업데이트한 후 다시 시도하세요.

카드를 보호하기 위해 계정당 시간당 제한된 횟수의 충전 시도만 가능합니다. `Too many top-up attempts. Wait a while and try again.` 메시지가 표시되면 잠시 기다린 후 다시 시도하세요.

결제는 Stripe에서 처리합니다. FastComments는 카드 정보를 절대 저장하지 않습니다.