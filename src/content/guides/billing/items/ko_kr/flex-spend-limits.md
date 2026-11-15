Flex에는 고정 상한이 없으므로, 트래픽 급증이나 통제되지 않은 통합으로 인해 예상보다 큰 청구서가 발생할 수 있습니다. 지출 한도를 설정하면 청구서의 사용량 기반 부분에 상한을 둘 수 있습니다.

[Billing Info](https://fastcomments.com/auth/my-account/billing-info) 페이지에서 플랜 요약에 `Monthly spend limit`이 표시됩니다. `Edit`을 클릭하여 대화 상자를 엽니다. `Target monthly max`를 입력하고 FastComments가 개별 한도를 이에 맞게 설정하도록 하거나, 각 한도를 직접 설정할 수 있습니다:

- `Page loads per month`
- `Comments stored`
- `SSO users`
- `API credits per month`
- `Small widget credits per month`

필드를 비워 두면 해당 유형의 사용량은 무제한으로 유지됩니다. 대화 상자에는 입력한 한도에 대한 `Estimated monthly max`가 표시되며, Flex 최소 금액은 여전히 적용됩니다.

### 한도의 작동 방식

한도에 도달하면 해당 유형의 사용량이 월의 나머지 기간 동안 차단됩니다. 페이지 로드의 경우, 위젯이 댓글 로드를 중단하고 월이 바뀌거나 한도를 올릴 때까지 `This FastComments.com account has exceeded its limits.`를 표시합니다. 한도를 올리면 몇 분 이내에 적용됩니다.

지출 한도는 전체 청구서에 대한 하드 캡이 아닙니다. 위의 사용량 항목만 포함합니다. 운영자나 추가 도메인 같이 직접 추가하는 시트와 사이트는 직접 관리하는 항목이므로 여전히 청구됩니다. [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing)에서 다음 청구서 미리보기를 언제든지 확인할 수 있습니다.

`Account Owner` 또는 `Billing Admin`만 지출 한도를 변경할 수 있습니다.