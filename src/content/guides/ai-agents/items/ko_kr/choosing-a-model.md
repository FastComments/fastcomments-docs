각 에이전트는 편집 양식의 **Model** 섹션에서 선택된 두 LLM 모델 중 하나로 실행됩니다.

### The two options

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - 기본값입니다. 더 높은 추론 품질을 제공하지만 호출당 다소 느립니다. 잘못된 호출의 비용이 큰 상황(예: `Moderator` 템플릿이나 `ban_user` 또는 `mark_comment_spam`를 호출하는 작업)에 권장됩니다.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - 호출당 더 빠르고 지연 시간이 낮습니다. 응답을 몇 초 내에 받아야 하고 잘못된 호출의 결과가 경미한 대량 처리형 에이전트(예: 환영 인사, 스레드 고정기)에 권장됩니다.

두 모델 모두 함수 호출(function calling)을 지원하며 동일한 OpenAI 호환 API를 통해 실행되고 동일한 툴별 스키마를 공유하므로 저장된 에이전트를 언제든지 다른 구성 변경 없이 모델 간에 전환할 수 있습니다.

### Cost differences

두 모델은 토큰당 비용이 다릅니다. 에이전트의 [budget caps](#budgets-overview)는 토큰이 아닌 귀하의 계정 통화로 표시되므로 한 모델에서 다른 모델로 전환하면 일일 및 월간 한도 내에서 실행 횟수가 어떻게 달라지는지에 영향을 줍니다. [Run History](#run-history) 페이지는 실행이 완료된 후 통화 단위의 실행당 비용을 보여줍니다 — 전환 후 몇 번의 실행을 지켜보는 것이 새로운 소모 속도를 가늠하는 가장 쉬운 방법입니다.

### Tokens per run

모델의 응답 토큰 사용량은 모든 트리거에서 **tokensUsed**로 기록됩니다. 이는 `trigger.succeeded` 및 `trigger.failed` 웹후크 페이로드에 포함되어 있습니다(자세한 내용은 [Webhook Payloads](#webhook-payloads) 참조) 및 [Run Detail View](#run-detail-view)에 표시됩니다. 사용량은 다음에 따라 달라집니다:

- 포함하는 [Context](#context-options)의 양 — 스레드 컨텍스트, 사용자 히스토리, 페이지 메타데이터는 모두 토큰을 추가합니다.
- [Initial prompt](#personality-prompt) 및 [Community guidelines](#community-guidelines)의 길이.
- 단일 실행에서 에이전트가 호출하는 툴의 수(각 툴 호출과 그 결과가 모델을 왕복합니다).

**Max Tokens Per Trigger** (기본값 20,000)은 에이전트별로 설정되는 실행당 상한입니다.

### Switching models

편집 양식에서 언제든지 모델을 전환할 수 있습니다. 기존의 실행 기록과 분석은 실행 시점의 원래 토큰 및 비용 수치를 유지합니다. 새 모델은 저장한 이후에 시작되는 실행에만 적용됩니다.

"더 저렴한 모델을 자동으로 사용"하는 옵션은 없습니다. 선택은 에이전트별로 명시적입니다.