An **approval**은 플랫폼이 실행하기 전에 인간의 승인 또는 거부가 필요한 대기열에 들어간 도구 호출입니다.

### Configuring approvals

에이전트 편집 양식의 **Approvals** 섹션에는 에이전트가 호출할 수 있도록 허용된 모든 도구(허용 목록)가 나열되며, 인간의 검토가 필요한 도구에 체크할 수 있습니다. 체크하지 않은 도구는 즉시 실행됩니다. 체크된 도구는 대기열에 들어갑니다.

허용되지 않은 도구는 *즉시 거부*되며, 대기열에 들어가지 않습니다 — 승인 조건은 다른 면에서 허용된 도구에만 적용됩니다.

### What happens when a gated action fires

1. 에이전트가 인수, 정당화(justification), 및 신뢰도(confidence)와 함께 도구 호출(예: `ban_user`)을 선택합니다.
2. 실행하는 대신 플랫폼은 도구 이름, 인수, 정당화, 신뢰도, 그리고 그것을 생성한 트리거를 설명하는 컨텍스트 스냅샷과 함께 `PENDING` 상태의 승인을 대기열에 추가합니다.
3. 검토자에게 알림이 전송됩니다(자세한 내용은 [Approval Notifications](#approval-notifications) 참조).
4. 에이전트의 실행은 완료되어 기록되며 - 해당 작업은 [Run Detail View](#run-detail-view)에서 **Pending approval**로 표시됩니다.

### Reviewing approvals

승인 인박스는 [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals)에 있으며, 대기 중인 항목, 승인된 항목, 거부된 항목, 실행 실패한 항목을 나열합니다. 각 항목에는 다음이 표시됩니다:

- **Tool name and arguments** - 에이전트가 정확히 수행하려는 내용입니다.
- **Agent reasoning** - 에이전트가 제공한 정당화입니다.
- **Confidence** - 에이전트가 자체 평가한 신뢰도(0.0~1.0).
- **Context snapshot** - 작업의 대상이 되는 댓글, 페이지 및 사용자입니다.

두 개의 버튼: **Approve** 및 **Reject**. Approve는 실제로 도구를 실행하고, Reject는 폐기합니다.

### Approval status states

승인은 다음 상태를 거칩니다:

| State | Meaning |
|---|---|
| `PENDING` | 인간의 결정을 기다리는 중입니다. |
| `APPROVED` | 사람이 승인했으며 작업이 실행되었습니다. |
| `REJECTED` | 사람이 거부했습니다. 작업은 실행되지 않았습니다. |
| `EXECUTION_FAILED` | 사람이 승인했지만 작업이 실패했습니다(예: 대상 댓글이 이미 삭제된 경우). |
| `EXECUTING` | 일시적 상태: 사람이 Approve를 클릭했고 작업이 실행 중입니다. 동시에 승인 클릭이 일어날 때 직렬화를 위해 사용되며, 실제 부작용이 있는 도구가 두 번 실행되는 일을 방지합니다. |

두 검토자가 동시에 Approve를 클릭할 때 `EXECUTING` 상태가 중요합니다 - 한 명이 승리하고, 다른 사람은 승인이 이미 이동했음을 보게 됩니다.

### What gets cleaned up

대기 중인 승인은 결정될 때까지 그대로 남아 있습니다. 생성 후 **90일**이 지나면 자동 만료됩니다. 다른 어떤 상태에 있는 승인도 저장 관리를 위해 90일 후에 정리됩니다.

승인의 "decided by" / "decided at" / "executed at" / "execution result" 필드는 승인이 상태를 이동하면서 채워지며 — 모두 인박스 상세 보기에서 볼 수 있습니다.

### Webhook integration

승인이 이동할 때 두 개의 웹후크 이벤트가 발생합니다:

- **`approval.requested`** - PENDING이 삽입될 때.
- **`approval.decided`** - APPROVED, REJECTED 또는 EXECUTION_FAILED로 전환될 때.

둘 다 다른 모든 웹후크와 동일하게 서명됩니다. [Webhook Events](#webhook-events) 및 [Webhook Payloads](#webhook-payloads)를 참조하세요.

### Hardening: known-tool gate

승인은 인식된 에이전트 도구가 아닌 도구 이름을 대기열에 추가하는 것을 거부합니다. 이는 심층 방어를 위한 검사입니다: 향후 어떤 코드 경로가 LLM에서 유래된 도구 이름을 승인 흐름에 전달하더라도, 그 문자열은 대기열된 승인으로 절대 들어갈 수 없습니다. 알려진 도구 이름의 집합은 [Tools Reference](#tools-overview)에 나열된 동일한 집합입니다.

### Common gating patterns

- **Brand-new moderation agent** - `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`을 게이트하세요. 몇 주 동안 인박스를 주시한 다음 오류가 적은 도구에 대해서는 게이트를 해제하세요.
- **Long-term moderation agent** - `ban_user`와 되돌릴 수 없는 작업들(`deleteAllUsersComments`, `banIP`)은 영구적으로 게이트 상태로 유지하세요.
- **EU region** - Article 17에 따라 `ban_user`는 사용자가 무엇을 선택하든 잠겨 있습니다. [EU DSA Article 17 Compliance](#eu-dsa-compliance)를 참조하세요.

### What approvals do **not** do

- 에이전트의 다른 도구 호출을 지연시키지 않습니다. 에이전트의 실행은 여러 도구를 호출할 수 있으며, 게이트된 도구만 대기열에 들어갑니다 - 나머지는 정상적으로 실행됩니다.
- 인간이 거부해도 에이전트의 실행을 롤백하지 않습니다. 실행의 비게이트된 부분은 이미 완료되어 있습니다.