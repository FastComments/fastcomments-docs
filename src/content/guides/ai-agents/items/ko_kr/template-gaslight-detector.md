**Template ID:** `gaslight_detector`

The Gaslight Detector는 대화 중간에 기록을 바꾸는 댓글 수정을 감시합니다 - 작성자가 이전 댓글의 의미를 바꿔 답글들이 문맥에 맞지 않거나 틀려 보이게 만드는 유형입니다. 에이전트가 해당 수정을 그 한계를 넘었다고 판단하면 원래 텍스트를 복원하고 작성자에게 DM으로 설명합니다.

이 템플릿은 사용자 콘텐츠를 수정하기 때문에 위험도가 더 높습니다. 읽기 전용 템플릿보다 [dry-run](#dry-run-mode)에서 더 오래 실행하고, 모델의 판단을 신뢰할 때까지 `edit_comment`를 [approval](#approval-workflow) 뒤에 둬야 합니다.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - 에이전트는 새 텍스트와 이전 텍스트를 비교하여 해당 수정이 이미 존재하는 답글들을 왜곡하는지 판단합니다.

전체 페이로드(이전 댓글 텍스트 및 수정 시점의 답글 수 포함)는 [Trigger: Comment Edited](#trigger-comment-edit)를 참조하세요.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - 수정이 가스라이팅으로 판단될 때 원래 텍스트를 복원하는 데 사용합니다.
- [`warn_user`](#tool-warn-user) - 사용자가 다음 방문 시 보는 소프트 경고를 발행합니다.
- [`send_dm`](#tools-overview) - 설명 채널; 사용자는 수정이 되돌려진 이유를 설명하는 다이렉트 메시지를 받습니다.

이 에이전트는 차단, 스팸 표시, 투표, 또는 새 댓글 게시를 할 수 없습니다 - 적용 범위를 의도적으로 좁게 설정했습니다.

### Recommended additions before going live

- **`edit_comment`를 [approval](#approval-workflow) 뒤로 두세요.** 댓글 복원은 작성자와 편집된 버전을 본 모든 사람에게 노출되므로, 오탐은 난처합니다. 에이전트가 일관되게 동작한다는 것이 dry-run에서 확인될 때까지 승인 절차를 유지하세요.
- **사이트에서 무엇이 가스라이팅에 해당하는지 프롬프트를 구체화하세요.** 기본 프롬프트는 의도적으로 짧습니다. 모델에 구체적 예시를 제공하세요 - "예/아니오 주장의 뒤집기", "답글이 인용한 숫자 삭제", "답글이 달린 후 적대적인 문장 추가" - 그리고 오예외 사례도 명시하세요(예: 오타 수정, 서식 정리, 출처 추가).
- **트리거 컨텍스트의 답글 수를 사용하세요.** 답글이 없는 댓글에 대한 수정은 대화를 왜곡할 수 없으므로, 프롬프트에서 그런 경우를 건너뛰라고 모델에 지시해야 합니다.
- **[Context Options](#context-options)에서 "Include commenter's trust factor, account age, ban history, and recent comments"를 선택하세요.** 장기간 신뢰할 수 있는 계정을 볼 수 있을 때 모델은 훨씬 덜 공격적입니다.
- **프롬프트에 짧은 편집 유예 시간을 고려하세요.** 많은 수정이 처음 30–60초 내에 이루어지며 오타 수정인 경우가 많습니다; 모델에 그만큼 빠른 수정은 무시하라고 지시하세요.

### Recommended dry-run window

실제 트래픽으로 최소 2주간 [dry-run](#dry-run-mode)으로 실행한 뒤 Enabled로 전환하고, 해당 기간 동안 플래그된 모든 수정을 검토하세요. 실생성 전 에이전트를 마지막 30일간의 수정에 대해 재생할 때는 [Test Runs (Replays)](#test-runs-replays)를 사용하세요.