---
에이전트에는 세 가지 상태가 있습니다:

### Disabled

에이전트가 꺼져 있습니다. 트리거는 처리되지 않으며 에이전트는 디스패치 경로에 나타나지 않습니다. 실행 기록, 분석 및 메모리는 그대로 남아 있으며 — 나중에 다시 활성화하면 이전 데이터가 그대로 있습니다.

Use `Disabled` when:
- 에이전트를 잃지 않고 로테이션에서 제외하려는 경우.
- 에이전트가 이상 동작을 하고 있어 조사하는 동안 즉시 중지해야 하는 경우.
- 에이전트를 계절적으로 교체하는 경우(예: 휴일 전용 인사담당자).

### Dry Run - default for new agents

에이전트는 엔드투엔드로 실행됩니다 - 트리거를 처리하고, LLM을 호출하고, 도구 호출을 선택하며, 정당화 및 신뢰도를 계산하지만 - **실제 조치는 이루어지지 않습니다**. 각 실행은 [실행 기록](#run-history)에 **Dry Run** 배지로 기록됩니다.

Use `Dry Run` when:
- 새 에이전트가 막 출시되었을 때. 모든 시작 템플릿은 dry-run 상태로 시작합니다.
- 프롬프트를 편집했거나 트리거 집합을 변경하여 커밋하기 전에 변경 사항이 어떻게 작동하는지 확인하고 싶을 때.
- [테스트 실행 / 재실행](#test-runs-replays)을 실행하는 경우(재실행은 에이전트 상태와 상관없이 dry-run을 강제합니다).

플랫폼은 dry-run 실행에 대해서도 토큰을 과금합니다 - LLM 호출은 여전히 발생하며, 부수 효과만 건너뜁니다. 예산 상한은 dry-run에도 적용됩니다. 자세한 내용은 [예산 개요](#budgets-overview)를 참조하세요.

### Enabled

에이전트가 실제 작업을 수행합니다. 도구 호출이 실행되거나, 작업이 게이트된 경우 [승인](#approval-workflow) 대기열에 쌓입니다.

Use `Enabled` after dry-run output looks correct.

### Switching status

편집 폼에서 어느 두 상태든 전환할 수 있습니다. Switching from Dry Run to Enabled does not retroactively re-execute the dry-run actions - those stay as dry-run history. 그 시점 이후의 새 트리거는 라이브로 실행됩니다.

Enabled에서 Disabled로 도중에 전환해도 진행 중인 실행을 **중단하지는 않습니다**. 현재 실행 중인 트리거는 이미 시작한 작업을 완료합니다(이미 시작한 작업에 한해); 다음 트리거는 에이전트가 이제 Disabled이므로 취소됩니다.

### Status during billing problems

테넌트의 결제가 무효화되면, 저장된 상태와 관계없이 모든 에이전트는 사실상 일시 중지됩니다 - 결제가 복구될 때까지 트리거는 `BILLING_INVALID`로 드랍됩니다. 저장된 상태 필드는 변경되지 않으며, 디스패처가 실행을 거부할 뿐입니다. 자세한 내용은 [요금제 및 자격](#plans-and-eligibility)을 참조하세요.
---