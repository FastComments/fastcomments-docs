에이전트 웹훅 이벤트 타입은 네 가지가 있습니다. 각 이벤트에는 숫자형 enum 값(페이로드에 사용)과 표준 문자열 이름(`event` 봉투 필드 및 `X-FastComments-Agent-Event` HTTP 헤더에 사용)이 있습니다.

| 이벤트 이름 | Enum | 발생 시점 |
|---|---|---|
| `trigger.succeeded` | 0 | 에이전트 실행이 `SUCCESS` 상태로 완료될 때. |
| `trigger.failed` | 1 | 에이전트 실행이 `ERROR` 상태로 완료될 때. |
| `approval.requested` | 2 | 승인이 `PENDING` 상태로 대기열에 추가될 때. |
| `approval.decided` | 3 | 승인이 `APPROVED`, `REJECTED`, 또는 `EXECUTION_FAILED`로 전환될 때. |

### `trigger.succeeded`

에이전트의 실행이 오류 없이 완료된 후 발생합니다. 페이로드의 `data` 필드에는 다음이 포함됩니다:

- `triggerId` - 고유한 실행 ID.
- `triggerType` - 실행을 시작한 [트리거 사유 enum](#triggers-overview).
- `status` - `SUCCESS` (문자열).
- `tokensUsed` - 이 실행에서 소비된 토큰 수.
- `wasDryRun` - 에이전트가 [드라이런 모드](#dry-run-mode)였으면 true.
- `actions` - `TenantAgentAction` 레코드 배열 ([웹훅 페이로드](#webhook-payloads) 참조).
- `commentId`, `url`, `urlId` - 트리거에 해당 값이 있었던 경우.

실행에서 아무 작업도 수행하지 않았다면 `actions` 배열은 비어 있습니다 - 이는 "에이전트가 아무것도 하지 않기로 결정했다"는 성공적인 실행이며, 이를 아는 것이 유용합니다.

### `trigger.failed`

실행 중 오류가 발생할 때 발생합니다. `trigger.succeeded`와 동일한 페이로드 구조이며, `status: 'ERROR'`와 무엇이 잘못되었는지 설명하는 추가 `errorMessage` 필드가 있습니다. 가능한 오류로는 LLM 호출 실패, 도구 디스패치 실패, 실행 중 예산 소진 등이 있습니다.

`actions`에는 오류 발생 전에 완료된 도구 호출 항목이 여전히 포함될 수 있습니다.

### `approval.requested`

승인이 `PENDING` 상태로 대기열에 추가되는 즉시 발생합니다. 페이로드에는 다음이 포함됩니다:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - LLM 호출에서 **그대로 전달된** 도구의 인수. 구조는 도구별로 다르며 안정적인 공개 계약이 아닙니다 - 새로운 도구가 추가됨에 따라 스키마가 변경될 수 있습니다.
- `createdAt`.
- `justification`, `confidence` - 에이전트가 제공한 경우.
- `contextSnapshot` - 승인과 관련된 댓글/페이지 컨텍스트.

대기 중인 승인을 채팅 운영 채널로 전달하는 데 유용합니다: `approval.requested`를 구독하는 Slack 봇은 액션과 그 근거를 모더레이션 채널에 게시하여 한눈에 검토할 수 있게 합니다.

### `approval.decided`

승인이 `PENDING`에서 벗어날 때 발생합니다. 페이로드에는 다음이 포함됩니다:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, 또는 `EXECUTION_FAILED`.
- `decidedBy` - 결정을 내린 모더레이터의 사용자 ID.
- `decidedAt` - 결정 시각.
- `executedAt` - APPROVED인 경우, 플랫폼이 승인된 액션을 실행한 시각.
- `executionResult` - APPROVED인 경우, 실행기의 결과를 설명하는 문자열.
- `contextSnapshot` - 댓글/페이지 컨텍스트.

이 이벤트는 모든 결정 결과를 다룹니다:

- **승인 + 정상 실행** -> `status: APPROVED`, `executedAt` 설정됨, `executionResult`는 성공 메시지.
- **승인 + 실행기 실패** -> `status: EXECUTION_FAILED`, `executedAt` 설정됨, `executionResult`는 실패 내용 설명.
- **거부** -> `status: REJECTED`, `executedAt`은 null, `executionResult`는 null.

### 헤더

모든 전송에는 이벤트의 표준 문자열 이름(`trigger.succeeded` 등)이 담긴 `X-FastComments-Agent-Event` HTTP 헤더가 포함됩니다. 여러 이벤트 타입을 처리하는 단일 URL 엔드포인트를 사용하는 경우 유용합니다.

### 참고

- 이벤트별 전체 페이로드 스키마는 [웹훅 페이로드](#webhook-payloads).
- HMAC 방식은 [웹훅 서명](#webhook-signing).
- 전송 의미론은 [웹훅 재시도](#webhook-retries).