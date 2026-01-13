FastComments는 각 댓글에 대해 상세한 이벤트를 자동으로 추적하여 중재 결정 및 시스템 동작의 투명성을 제공합니다. 이 로그는 댓글이 왜 승인되었는지, 스팸으로 표시되었는지, 또는 상태가 변경되었는지 이해하는 데 도움이 됩니다.

특정 댓글을 선택하여 댓글 중재 대시보드에서 개별 댓글의 로그를 볼 수 있습니다.

## 댓글 로그 이벤트

각 댓글은 생애주기 동안 발생한 이벤트의 로그를 유지합니다. 아래는 추적되는 이벤트 유형들입니다:

### 익명화 이벤트
- **Anonymized** - 댓글 내용이 지워지고 사용자가 삭제된 것으로 표시됨

### 승인 이벤트
- **ApprovedDueToPastComment** - 사용자가 이전에 승인된 댓글을 보유하고 있어 댓글이 승인됨
- **ApprovedIsAdmin** - 사용자가 관리자여서 댓글이 승인됨
- **NotApprovedRequiresApproval** - 댓글이 수동 승인을 필요로 함

### 스팸 탐지 이벤트
- **IsSpam** - 탐지 엔진에 의해 댓글이 스팸으로 표시됨
- **IsSpamDueToBadWords** - 비속어 필터로 인해 댓글이 스팸으로 표시됨
- **IsSpamFromLLM** - AI/LLM 엔진에 의해 댓글이 스팸으로 표시됨
- **IsSpamRepeatComment** - 반복적이라는 이유로 댓글이 스팸으로 표시됨
- **NotSpamIsOnlyImage** - 이미지만 포함되어 있어 스팸으로 표시되지 않음
- **NotSpamIsOnlyReacts** - 반응(리액션)만 포함되어 있어 스팸으로 표시되지 않음
- **NotSpamNoLinkOrMention** - 의심스러운 링크나 멘션이 없어 스팸으로 표시되지 않음
- **NotSpamPerfectTrustFactor** - 높은 사용자 신뢰도로 인해 스팸으로 표시되지 않음
- **NotSpamTooShort** - 분석하기에 너무 짧아 스팸으로 표시되지 않음
- **NotSpamSkipped** - 스팸 검사가 건너뛰어짐
- **NotSpamFromEngine** - 탐지 엔진에 의해 스팸이 아니라고 판단됨

### 비속어/욕설 이벤트
- **BadWordsCheckFailed** - 비속어 필터 검사 중 오류 발생
- **BadWordsFoundBadPhrase** - 비속어 필터가 부적절한 구문을 감지함
- **BadWordsFoundBadWord** - 비속어 필터가 부적절한 단어를 감지함
- **BadWordsNoDefinitionForLocale** - 댓글 언어에 대한 비속어 정의가 없음

### 사용자 인증 이벤트
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 댓글이 인증을 필요로 하지만 사용자가 인증된 세션에 없음
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 댓글이 인증을 필요로 하지만 사용자가 아직 인증되지 않음
- **InVerifiedSession** - 댓글을 작성하는 사용자가 인증된 세션에 있음
- **SentVerificationEmailNoSession** - 인증되지 않은 사용자에게 인증 이메일이 전송됨
- **SentWelcomeEmail** - 신규 사용자에게 환영 이메일이 전송됨

### 신뢰 및 보안 이벤트
- **TrustFactorChanged** - 사용자의 신뢰 지수가 변경됨
- **SpamFilterDisabledBecauseAdmin** - 관리자 사용자에 대해 스팸 필터가 우회됨
- **TenantSpamFilterDisabled** - 테넌트 전체에 대해 스팸 필터가 비활성화됨
- **RepeatCommentCheckIgnored** - 반복 댓글 검사가 우회됨
- **UserIsAdmin** - 사용자가 관리자임이 식별됨
- **UserIsAdminParentTenant** - 사용자가 상위 테넌트 관리자임이 식별됨
- **UserIsAdminViaSSO** - SSO를 통해 사용자가 관리자임이 식별됨
- **UserIsMod** - 사용자가 중재자(모더레이터)임이 식별됨

### 댓글 상태 변경
- **ExpireStatusChanged** - 댓글 만료 상태가 변경됨
- **ReviewStatusChanged** - 댓글 검토 상태가 변경됨
- **SpamStatusChanged** - 댓글 스팸 상태가 업데이트됨
- **ApproveStatusChanged** - 댓글 승인 상태가 변경됨
- **TextChanged** - 댓글 텍스트 내용이 수정됨
- **VotesChanged** - 댓글 투표 수가 업데이트됨
- **Flagged** - 댓글이 사용자들에 의해 신고(플래그)됨
- **UnFlagged** - 댓글 신고(플래그)가 제거됨

### 중재 작업
- **Pinned** - 댓글이 중재자에 의해 고정됨
- **UnPinned** - 댓글이 중재자에 의해 고정 해제됨
- **RestoredFromAnonymized** - 댓글이 익명화 상태에서 복원됨

### 알림 이벤트
- **CreatedNotifications** - 댓글에 대한 알림이 생성됨
- **NotificationCreateFailure** - 알림 생성에 실패함
- **BadgeAwarded** - 댓글에 대해 사용자 배지가 수여됨

### 게시 이벤트
- **PublishedLive** - 댓글이 라이브 구독자들에게 게시됨

### 통합 이벤트
- **WebhookSynced** - 댓글이 웹훅을 통해 동기화됨

### 스팸 규칙 이벤트
- **SpamRuleMatch** - 댓글이 사용자 정의 스팸 규칙과 일치함

## 댓글 로그에 접근하기

댓글 로그는 자동으로 생성되어 각 댓글과 함께 저장됩니다. 이들은 다음과 같은 귀중한 인사이트를 제공합니다:

- 중재 결정 이해
- 승인/스팸 문제 디버깅
- 사용자 행동 패턴 추적
- 시스템 동작 감사

이 로그들은 중재 과정의 투명성을 유지하는 데 도움을 주며, 댓글 시스템의 동작을 미세 조정하는 데 도움을 줍니다.