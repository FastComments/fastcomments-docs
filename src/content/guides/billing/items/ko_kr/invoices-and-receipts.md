플랜 갱신, Flex 월 사용료, 플랜 변경 등 모든 청구 건에 대해 인보이스(청구서)가 생성됩니다. 인보이스는 결제 완료 후 영수증 역할을 합니다.

### 인보이스 찾기

[Billing History](https://fastcomments.com/auth/my-account/billing-history)를 열거나, Billing Info 페이지에서 `View Invoices`를 클릭하세요. 표에는 각 인보이스의 날짜, 청구 기간, 인보이스 번호, 설명, 총액, 상태(`Paid`, `Unpaid`, `Draft` 또는 `Void`), 만기일이 최신순으로 페이지당 10개씩 표시됩니다.

각 인보이스에 대해:

- `View`는 Stripe의 호스팅 페이지에서 인보이스를 열며, 거기서 다운로드할 수도 있습니다.
- `PDF`는 PDF를 직접 다운로드합니다.
- `Pay Now`는 미결제 인보이스에서 `View` 대신 표시되며, 해당 인보이스의 Stripe 결제 페이지로 이동하여 직접 결제할 수 있습니다.

인보이스는 생성될 때 청구 이메일로도 발송됩니다. 계정 소유자 대신 재무 사서함으로 받으려면 [Invoice Settings](#invoice-settings)에서 `Billing Email`을 설정하세요.

### 인보이스에 포함되는 내용

인보이스에는 [Invoice Settings](#invoice-settings)의 청구 이름과 주소, VAT / Tax ID, 라인 항목, 적용된 선불 [크레딧](#credits), 카드로 청구된 금액이 표시됩니다. Flex 인보이스는 사용 유형별로 한 줄씩 표시되며, 자세한 내용은 [How Flex Usage Is Measured](#how-flex-usage-is-measured)를 참조하세요. 유럽 연합 내 청구 주소의 경우 Flex 인보이스는 0% VAT로 발행되며 바닥글에 `Reverse Charge Transaction: Customer to pay VAT`가 표시됩니다.

인보이스 설정 변경 사항은 저장한 이후에 생성되는 인보이스에 적용됩니다. 이미 발행된 인보이스는 수정할 수 없으므로, 다음 청구일 전에 회사 정보와 VAT / Tax ID를 설정하세요.

### 다른 곳에서 청구되는 계정

청구가 외부에서 처리되는 경우(예: 지원팀과 협의하여 인보이스로 진행), 카드로 청구되는 대신 결제용 인보이스가 발송되며 Billing History에 표시되지 않을 수 있습니다. Shopify 또는 BigCommerce를 통해 설치된 계정은 해당 플랫폼에서 청구되며, Billing Info 페이지는 해당 플랫폼의 플랜 페이지로 연결됩니다. 상위 계정을 통해 청구되는 하위 계정은 자체 인보이스가 없습니다. 자세한 내용은 [When You Are Charged](#when-you-are-charged)를 참조하세요.