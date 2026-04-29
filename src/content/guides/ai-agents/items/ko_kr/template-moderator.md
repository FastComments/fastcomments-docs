**템플릿 ID:** `tos_enforcer`

The Moderator template is the recommended starting point if your goal is reducing manual moderation load. It reviews new and flagged comments and applies your community rules.

### 내장 초기 프롬프트

[inline-code-attrs-start title = '모더레이터 템플릿 초기 프롬프트'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

You will almost always want to **augment this prompt** with concrete examples of what your site does and does not allow. The platform's own escalation policy (warn before ban, search memory before banning) is already baked into the system prompt the agent receives, so you do not need to repeat it.

### 트리거

- **새 댓글 게시됨** (`COMMENT_ADD`) - 에이전트가 모든 새 댓글을 검사합니다.
- **댓글이 플래그 임계값을 넘김** (`COMMENT_FLAG_THRESHOLD`, 기본 임계값: 3) - 다른 사용자가 플래그한 댓글을 에이전트가 재평가합니다.

### 허용된 도구

- [`mark_comment_approved`](#tools-overview) - 사전 중재(프리모더레이션) 환경에서 에이전트가 깨끗한 댓글을 공개하고 나머지를 숨기는 데 유용합니다.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

이 에이전트는 댓글을 게시하거나, 투표하거나, 고정하거나, 잠그거나, 뱃지를 수여하거나, 이메일을 보낼 수 없습니다 - 프롬프트는 의도적으로 권한이 제한되어 있습니다.

### 라이브 이전 권장 추가 사항

- **[커뮤니티 가이드라인](#community-guidelines)을 설정하세요.** 몇 문장의 서면 정책이면 충분합니다; 에이전트는 매 실행마다 이를 적용합니다.
- **`ban_user`를 [승인](#approval-workflow) 뒤에 배치하세요.** 이는 EU 지역에서 기본적으로 활성화되어 있습니다(자세한 내용은 [EU DSA Article 17 Compliance](#eu-dsa-compliance) 참조) 및 모든 지역에서 권장됩니다.
- **콘텐츠 양은 적지만 위험도가 높은 경우 `mark_comment_spam`도 승인 뒤에 두는 것을 고려하세요.**
- **사전 중재를 운영하는 경우 `mark_comment_approved`를 승인 뒤에 두세요.** 잘못된 댓글을 승인하면 독자들에게 노출되므로, 에이전트가 드라이런으로 신뢰를 얻을 때까지 승인 권한을 제한하세요.
- **[Context Options](#context-options)에서 "댓글 작성자의 신뢰도, 계정 연령, 정지 기록 및 최근 댓글 포함"을 선택하세요.** 모델은 사용자가 오랜 기간 선의로 활동한 사용자임을 확인할 수 있을 때 훨씬 덜 공격적으로 경고합니다.

### 권장 드라이런 기간

이 템플릿을 활성화(Enabled)로 전환하기 전에 실제 트래픽에 대해 최소 일주일 동안 [드라이런](#dry-run-mode)으로 실행하세요. 또한 지난 30일 동안을 미리보기하려면 [테스트 실행(재생)](#test-runs-replays)을 사용하세요.