카드에 언제 얼마가 청구되는지는 사용 중인 플랜의 종류에 따라 다릅니다.

### 고정 플랜

`Creators` 또는 `Pro` 구독을 시작하면 첫 달 또는 첫 해 요금이 `Pay Now & Start Subscription`을 클릭하는 시점에 브라우저에서 즉시 청구됩니다. 은행에서 결제 확인을 요구하는 경우 열리는 창에서 확인 절차를 완료하세요. 결제가 완료되어야 구독이 시작됩니다.

이후에는 구독한 날짜를 기준으로 매월 같은 날짜, 또는 `Pay Annually`를 선택한 경우 매년 같은 날짜에 갱신됩니다. 갱신 요금은 등록된 카드로 자동 청구되며 인보이스는 결제 이메일로 발송됩니다. 별도의 조치는 필요하지 않습니다.

연간 결제는 월간 결제보다 15% 저렴합니다. Flex는 연간 결제가 불가능합니다.

### Flex

Flex는 후불제로 청구됩니다. 즉, 한 달 동안 서비스를 사용하면 해당 월의 인보이스가 그 달의 마지막 날(UTC)에 생성되어 즉시 카드에 청구됩니다. 청구일은 항상 그 달의 마지막 날이며 변경할 수 없습니다.

인보이스는 해당 월의 사용량만 포함합니다. 가입한 달의 사용량은 청구되지 않으므로 첫 인보이스는 가입한 달의 다음 달 말에 최초로 발행됩니다.

사용량이 Flex 최소 금액 미만인 경우 `FastComments Flex. (minimum ... /mo)` 항목이 인보이스를 최소 금액까지 채웁니다. 현재 최소 금액은 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지와 [요금 페이지](https://fastcomments.com/traffic-pricing)에서 확인할 수 있습니다.

다음 인보이스 금액은 언제든지 확인할 수 있습니다. `Next Invoice Amount`는 [Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지에 있으며, 항목별 상세 내역은 [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) 페이지에서 확인할 수 있습니다. 사용량 계산 방식은 [Flex 사용량 측정 방법](#how-flex-usage-is-measured)에 설명되어 있습니다.

### 카드 저장은 청구를 의미하지 않습니다

결제 정보가 설정된 후에는 플랜을 변경하지 않는 한 카드를 저장하거나 업데이트해도 청구되지 않습니다. 미지불 인보이스가 있는 경우는 예외입니다. 양식에는 `You won't be charged now unless you change plans.`라고 표시되며, 카드는 다음 인보이스를 위해 Stripe에 저장될 뿐입니다. 아직 유효한 결제 정보가 없는 경우에만 양식에 `Your card will be charged now.`라고 표시되며, 이 경우에도 Flex 계정은 월말까지 청구되지 않습니다. 계정에 선불 [크레딧](#credits)이 있는 경우 카드가 청구되기 전에 각 인보이스에 먼저 적용됩니다.

### 관리형 하위 계정

계정이 하위 계정을 관리하는 경우(예: 사이트 또는 환경별로 하나씩, 또는 화이트라벨 테넌트), 부모 계정만 청구됩니다. 각 하위 계정의 사용량은 부모 계정의 Flex 인보이스에 하위 계정 이름이 접두사로 붙은 별도 항목으로 표시되며, 하위 계정은 부모 계정의 결제 수단과 청구 상태를 공유합니다.