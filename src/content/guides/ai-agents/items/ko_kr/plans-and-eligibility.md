AI Agents는 **Flex** 및 **Pro** 플랜에서 사용할 수 있습니다. Creator 플랜에는 포함되지 않습니다.

### 플랜 수준 제한

각 플랜 단계는 다음을 설정합니다:

- **기본 일일 및 월간 예산 상한**. 에이전트별로 이를 낮출 수 있으며, 계정 전체 상한을 높이려면 상한이 더 높은 플랜이 필요합니다. [Budgets Overview](#budgets-overview)를 참조하세요.

정확한 수치는 [pricing page](https://fastcomments.com/traffic-pricing)와 계정의 청구 페이지에 표시됩니다. 또한 에이전트 편집 양식에 인라인으로 표시되어 양식을 떠나지 않고도 상한을 확인할 수 있습니다.

FastComments Pro는 월 $200 상당의 AI 사용량을 포함합니다. Flex는 모든 모델에 대해 백만 토큰당 $20의 요금이 부과됩니다(현재 GLM 5.2 또는 gpt-oss-120B 중 하나).

### 청구가 유효해야 함

AI 에이전트는 테넌트에 **유효한 청구 정보**가 있을 때만 실행됩니다. 결제 수단이 무효화되면 모든 에이전트가 일시 중지되고 AI 에이전트 페이지에 배너가 표시되어 **Billing Admin** 역할을 가진 사람이 청구 정보를 업데이트하도록 안내합니다. 청구가 복구되면 에이전트는 자동으로 재개되며, 중단 기간 동안 발생한 트리거는 재생되거나 보충되지 않습니다.

이는 필수 전제 조건입니다: 토큰 사용량이 계정에 청구되므로, 작동하는 결제 수단이 없으면 플랫폼이 LLM 호출을 전송하지 않습니다.

### 에이전트를 관리할 수 있는 사람

에이전트 관리 페이지는 **Customization Admin** 대시보드 역할에 의해 제한됩니다. **Comment Moderator Admins**는 검토 및 승인 결정을 할 수 있지만([Approval Workflow](#approval-workflow) 참조) 에이전트를 생성하거나 편집할 수 없습니다. **Billing Admins**는 에이전트 접근 권한 여부와 관계없이 [budget alert emails](#budget-alerts)를 받습니다.