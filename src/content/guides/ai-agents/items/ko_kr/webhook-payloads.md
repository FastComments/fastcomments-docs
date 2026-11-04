모든 에이전트 웹훅 페이로드는 공통 엔벨로프를 공유하며, 이벤트별 `data` 블록을 추가합니다. 이 페이지에서는 각각의 전체 스키마를 설명합니다.

### 엔벨로프 (모든 이벤트)

이벤트 유형과 관계없이 모든 페이로드에는 다음과 같은 최상위 필드가 있습니다:

[inline-code-attrs-start title = '웹훅 엔벨로프 스키마'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - the matched domain for this delivery",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 timestamp",
  "data": { /* event-specific, see below */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` 스키마:

[inline-code-attrs-start title = '트리거 이벤트 데이터 스키마'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "triggerId": "string",
  "triggerType": 0,
  "status": "SUCCESS | ERROR",
  "tokensUsed": 1234,
  "wasDryRun": false,
  "actions": [
    {
      "type": 0,
      "commentId": "string - optional",
      "userId": "string - optional",
      "badgeId": "string - optional",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - present on trigger.failed",
  "url": "string - optional",
  "urlId": "string - optional",
  "commentId": "string - optional"
}
[inline-code-end]

`triggerType`은 [트리거 이벤트 목록](#triggers-overview)의 숫자형 enum입니다.

`actions[].type`은 [도구 목록](#tools-overview)의 숫자형 enum입니다.

`actions[].pending`은 액션이 실행되는 대신 [승인](#approval-workflow) 대기열에 추가되었을 때 `true`입니다.

### `approval.requested`

`data` 스키마:

[inline-code-attrs-start title = '승인 요청 데이터 스키마'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* per-tool, see below */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - optional, agent reasoning",
  "confidence": 0.85,
  "contextSnapshot": { /* the comment/page context the approval is about */ }
}
[inline-code-end]

**`args`** 객체는 LLM 도구 호출이 전달한 내용을 그대로 담고 있습니다. 형태는 도구에 따라 다릅니다:

- `ban_user`의 경우: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- `mark_comment_spam`의 경우: `{ commentId, isSpam }`.
- `write_comment`의 경우: `{ comment, urlId, parentId? }`.
- ...등등.

도구 인자 형태의 집합은 **안정적인 공개 계약이 아닙니다**. 도구는 향후 추가될 수 있으며 플랫폼은 args를 그대로 전달합니다. 소비자는 해당 도구를 명확히 이해하고 있지 않는 한 args를 불투명한 블롭으로 취급해야 합니다.

**`contextSnapshot`**은 승인이 대기열에 추가된 댓글, 페이지, 사용자 컨텍스트를 담습니다. 그 형태는 트리거의 컨텍스트 메시지와 동일합니다.

### `approval.decided`

`data` 스키마:

[inline-code-attrs-start title = '승인 결정 데이터 스키마'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - the userId of the moderator who decided",
  "decidedAt": "string - ISO 8601 - optional, only present once decided",
  "executedAt": "string - ISO 8601 - present when APPROVED and execution finished",
  "executionResult": "string - executor result message - present after execute",
  "contextSnapshot": { /* same as approval.requested */ }
}
[inline-code-end]

### TenantAgentAction 형태

트리거 페이로드의 `actions[]` 내부에서 각 액션은 다음과 같습니다:

[inline-code-attrs-start title = 'TenantAgentAction 스키마'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - optional",
  "userId": "string - optional",
  "badgeId": "string - optional",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

`type` enum 값은 `AgentActionType`과 일치합니다:

- 0: `WRITE_COMMENT`
- 1: `VOTE_COMMENT`
- 2: `PIN_COMMENT`
- 3: `UNPIN_COMMENT`
- 4: `LOCK_COMMENT`
- 5: `UNLOCK_COMMENT`
- 6: `MARK_COMMENT_REVIEWED`
- 7: `MARK_COMMENT_APPROVED`
- 8: `MARK_COMMENT_SPAM`
- 9: `AWARDED_BADGE`
- 10: `BAN_USER`
- 11: `SENT_EMAIL`
- 12: `WARNED_USER`
- 13: `SAVED_MEMORY`

`SEARCH_MEMORY`는 읽기 전용이고 감사되지 않기 때문에 `actions[]`에 나타나지 않습니다.

### `triggerType` enum 값

`AgentTriggerReasonType`:

- 0: `COMMENT_ADD`
- 1: `COMMENT_EDIT`
- 2: `COMMENT_DELETE`
- 3: `COMMENT_PIN`
- 4: `COMMENT_UNPIN`
- 5: `COMMENT_LOCK`
- 6: `COMMENT_UNLOCK`
- 7: `COMMENT_VOTE_THRESHOLD`
- 8: `MODERATOR_REVIEWED_COMMENT`
- 9: `MODERATOR_APPROVED_COMMENT`
- 10: `MODERATOR_SPAMMED_COMMENT`
- 11: `MODERATOR_AWARDED_BADGE`
- 12: `COMMENT_FLAG_THRESHOLD`
- 13: `NEW_USER_FIRST_COMMENT`
- 14: `COMMENT_AUTO_SPAMMED`
- 15: `REPLAY` (내부용; 웹훅으로 전달되지 않음)

### 헤더

모든 전달에는 다음이 포함됩니다:

- `X-FastComments-Agent-Event` - 표준 이벤트 이름 (`trigger.succeeded` 등).
- `X-FastComments-Signature` - API 시크릿을 사용한 원시 본문의 HMAC-SHA256. [웹훅 서명](#webhook-signing)을 참조하세요.

### 안정성

엔벨로프 필드와 이벤트별로 문서화된 `data` 필드는 공개 계약의 일부입니다. 기존 페이로드에 새로운 선택적 필드를 추가하는 것은 허용되며 파괴적 변경으로 간주되지 않습니다 - 소비자는 알 수 없는 필드를 무시해야 합니다. `args`와 `contextSnapshot`의 형태는 계약의 일부가 **아닙니다**.