모든 에이전트에는 지출 한도가 있습니다. 한도에 도달하면 플랫폼이 에이전트의 디스패치를 중지하고 기간이 변경되면 재개합니다.

### 두 가지 범위, 두 가지 기간

총 4개의 한도가 있습니다 — 두 가지 범위(에이전트별, 테넌트별)와 두 가지 기간(일간, 월간)의 조합입니다.

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

트리거는 **네 가지 모든 한도**가 허용하는 경우에만 디스패치됩니다. 가장 먼저 소진되는 한도가 트리거를 중단시키는 원인입니다.

### 통화

에이전트별 예산은 계정 통화로 입력됩니다.

### 한도에 도달하면 무슨 일이 발생하나요

- 트리거는 `agentDaily` 또는 `tenantMonthly` 같은 [drop reason](#drop-reasons)과 함께 **dropped(중단됨)** 으로 기록됩니다.
- 중단된 횟수는 [Analytics page](#analytics-page) 의 "Triggers skipped (this month)"에 표시됩니다.
- LLM 호출이 이루어지지 않으므로 중단된 트리거 자체에는 토큰 비용이 발생하지 않습니다.
- 에이전트의 상태는 변경되지 않습니다 — 단지 기간이 갱신될 때까지 디스패치할 수 없습니다.

### 기간 갱신

- **일간** 한도는 UTC 자정에 리셋됩니다.
- **월간** 한도는 매 달 달력의 시작(1일) UTC에 리셋됩니다.

미사용 예산이 다음 기간으로 이월되지는 않습니다.

### 하드 캡 vs 소프트 경고

한도는 **하드** 입니다. "경고와 함께 10% 초과" 같은 모드는 없습니다. 한도에 도달하면 디스패치가 중지됩니다.

"소프트"한 부분은 [Budget Alerts](#budget-alerts) 이메일입니다 — 구성 가능한 임계값(기본값 80% 및 100%)에서 이메일을 받아 트래픽이 줄어들기 전에 한도를 올릴 수 있습니다.

### 현재 사용량을 확인하는 곳

- [Analytics page](#analytics-page) - 에이전트별 및 테넌트 전체 예산 사용량과 한도 표시.
- 에이전트 편집 폼의 **Stats** 섹션.
- 리스트 뷰(대기 중인 승인 수와 최근 실행 수가 에이전트 카드에 표시됨).

### 예산 설정 요령

몇 가지 경험 법칙:

- **새 에이전트** - 예산을 결정하세요. 1주일 동안 [Run History](#run-history)를 관찰하세요. 실행당 비용 × 예상 트리거 볼륨을 기반으로 조정합니다.
- **고빈도 에이전트**(예: 트래픽이 많은 사이트의 새 댓글 트리거) - 일일 한도가 무한 루프를 잡아냅니다. 정상적으로 바쁜 날이 무리 없이 포함되도록 예상 일일 지출의 2-3배 정도 일일 한도를 선택하세요.
- **요약기 또는 문맥 의존이 큰 에이전트** - 실행당 비용이 높습니다. 나쁜 하루가 월간 예산을 날리지 않도록 더 엄격한 일일 한도를 설정하세요.

### 재생(replay)에 대한 예산 우회

[Test runs / replays](#test-runs-replays)는 재생 폼에서 설정되는 자체의 **하드** 한도(에이전트의 일일/월간 한도와는 별도)를 적용받으며, 또한 에이전트 및 테넌트 한도에도 적용됩니다. 먼저 도달한 한도가 재생을 중단시킵니다.

### 참고

- 이메일 알림은 [Budget Alerts](#budget-alerts)를 참조하세요.
- 플랫폼이 토큰을 달러로 환산하는 방법은 [Cost Model](#cost-model)을 참조하세요.
- 트리거가 실행되지 않는 전체 이유 목록은 [Drop Reasons](#drop-reasons)를 참조하세요.