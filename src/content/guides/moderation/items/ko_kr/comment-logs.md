FastComments는 각 댓글에 대해 세부 이벤트를 자동으로 기록하여 조정 결정과 시스템 동작의 투명성을 제공합니다. 이러한 로그는 댓글이 승인되었거나 스팸으로 표시되었거나 상태가 변경된 이유를 이해하는 데 도움이 됩니다.

## 댓글 로그에 접근하기

특정 댓글의 로그를 보려면:

1. FastComments 대시보드에서 **Moderate Comments** 페이지로 이동합니다
2. 검사하려는 댓글을 찾습니다
3. 댓글의 작업 표시줄에서 **View Logs** 버튼(시계 아이콘)을 클릭합니다
4. 해당 댓글에 대한 전체 이벤트 기록이 표시된 대화 상자가 나타납니다

각 로그 항목에는 다음이 표시됩니다:
- **When** - 이벤트의 타임스탬프
- **Who** - 이벤트를 발생시킨 사용자 또는 시스템(해당되는 경우)
- **What** - 작업 또는 이벤트 유형
- **Details** - 이전/이후 값, 엔진 이름 또는 관련 데이터와 같은 추가 컨텍스트

## 댓글 로그 이벤트

각 댓글은 수명 주기 동안 발생하는 이벤트의 로그를 유지합니다. 아래는 추적되는 이벤트 유형입니다:

### Anonymization Events
- **Anonymized** - 댓글 내용이 지워지고 사용자가 삭제된 것으로 표시되었습니다
- **RestoredFromAnonymized** - 익명 처리된 상태에서 댓글이 복원되었습니다

### Approval Events
- **ApprovedDueToPastComment** - 사용자가 이전에 승인된 댓글을 가지고 있어 댓글이 승인됨(이전 댓글에 대한 참조 포함)
- **ApprovedIsAdmin** - 사용자가 관리자여서 댓글이 승인됨
- **NotApprovedRequiresApproval** - 댓글이 수동 승인을 필요로 함
- **NotApprovedLowTrustFactor** - 사용자 신뢰도 부족으로 댓글이 승인되지 않음(신뢰도 값 포함)

### Profile Comment Approval Events

이 이벤트들은 사용자 프로필의 댓글에 특히 적용됩니다:

- **ApprovedProfileAutoApproveAll** - 프로필 소유자가 모든 댓글에 대해 자동 승인을 활성화했기 때문에 프로필 댓글이 자동 승인됨
- **ApprovedProfileTrusted** - 댓글 작성자가 신뢰된 사용자여서 프로필 댓글이 승인됨(신뢰를 형성한 댓글에 대한 참조 포함)
- **NotApprovedProfileManualApproveAll** - 프로필 소유자가 수동 승인을 활성화했기 때문에 프로필 댓글이 수동 승인을 필요로 함
- **NotApprovedProfileNotTrusted** - 댓글 작성자가 신뢰되지 않아 프로필 댓글이 승인되지 않음
- **NotApprovedProfileNewUser** - 댓글 작성자가 신규 사용자여서 프로필 댓글이 승인되지 않음

### Spam Detection Events
- **IsSpam** - 감지 엔진에 의해 댓글이 스팸으로 표시됨(결정을 내린 엔진 포함)
- **IsSpamDueToBadWords** - 욕설 필터로 인해 댓글이 스팸으로 표시됨
- **IsSpamFromLLM** - AI/LLM 엔진에 의해 댓글이 스팸으로 표시됨(엔진 이름, 응답 및 토큰 수 포함)
- **IsSpamRepeatComment** - 반복적이라는 이유로 댓글이 스팸으로 표시됨(감지한 엔진 포함)
- **NotSpamIsOnlyImage** - 이미지만 포함되어 있어 스팸으로 표시되지 않음
- **NotSpamIsOnlyReacts** - 반응(리액션)만 포함되어 있어 스팸으로 표시되지 않음
- **NotSpamNoLinkOrMention** - 의심스러운 링크나 언급이 없어 스팸으로 표시되지 않음
- **NotSpamPerfectTrustFactor** - 사용자 신뢰도가 높아 스팸으로 표시되지 않음
- **NotSpamTooShort** - 분석하기에 너무 짧아 스팸으로 표시되지 않음
- **NotSpamSkipped** - 스팸 검사가 건너뛰어짐
- **NotSpamFromEngine** - 감지 엔진에 의해 스팸이 아니라고 판정됨(엔진 이름 및 신뢰도 포함)

### Bad Words/Profanity Events
- **BadWordsCheckFailed** - 욕설 필터 검사 중 오류 발생
- **BadWordsFoundBadPhrase** - 욕설 필터가 부적절한 문구를 감지함(문구 포함)
- **BadWordsFoundBadWord** - 욕설 필터가 부적절한 단어를 감지함(단어 포함)
- **BadWordsNoDefinitionForLocale** - 댓글 언어에 대한 욕설 정의가 없음(로케일 포함)

### User Verification Events
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 댓글은 인증이 필요하지만 사용자는 인증된 세션에 있지 않음
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 댓글은 인증이 필요하지만 사용자는 아직 인증되지 않음
- **InVerifiedSession** - 댓글 작성 사용자가 인증된 세션에 있음
- **SentVerificationEmailNoSession** - 인증되지 않은 사용자에게 인증 이메일이 전송됨
- **SentWelcomeEmail** - 신규 사용자에게 환영 이메일이 전송됨

### Trust and Security Events
- **TrustFactorChanged** - 사용자의 신뢰도(Trust Factor)가 변경됨(변경 전/후 값 포함)
- **SpamFilterDisabledBecauseAdmin** - 관리자 사용자에 대해 스팸 필터가 우회됨
- **TenantSpamFilterDisabled** - 테넌트 전체에 대해 스팸 필터가 비활성화됨
- **RepeatCommentCheckIgnored** - 반복 댓글 검사가 무시됨(이유 포함)
- **UserIsAdmin** - 사용자가 관리자(어드민)로 식별됨
- **UserIsAdminParentTenant** - 사용자가 상위 테넌트 관리자(어드민)로 식별됨
- **UserIsAdminViaSSO** - SSO를 통해 관리자로 식별됨
- **UserIsMod** - 사용자가 중재자(모더레이터)로 식별됨

### Comment Status Changes

상태 변경 이벤트는 변경 전/후 값과 변경을 수행한 사용자를 포함합니다:

- **ExpireStatusChanged** - 댓글의 만료 상태가 변경됨
- **ReviewStatusChanged** - 댓글 검토 상태가 변경됨
- **SpamStatusChanged** - 댓글의 스팸 상태가 업데이트됨
- **ApproveStatusChanged** - 댓글 승인 상태가 변경됨
- **TextChanged** - 댓글 텍스트 내용이 수정됨(수정 전/후 텍스트 포함)
- **VotesChanged** - 댓글 투표 수가 업데이트됨(상세 투표 내역 포함)
- **Flagged** - 댓글이 사용자에 의해 신고됨
- **UnFlagged** - 댓글 신고(플래그)가 제거됨

### Moderation Actions
- **Pinned** - 중재자가 댓글을 고정함(누가 고정했는지 포함)
- **UnPinned** - 중재자가 댓글 고정을 해제함(누가 해제했는지 포함)

### Notification Events
- **CreatedNotifications** - 댓글에 대한 알림이 생성됨(알림 수 포함)
- **NotificationCreateFailure** - 알림 생성에 실패함
- **BadgeAwarded** - 댓글에 대해 사용자 배지가 수여됨(배지 이름 포함)

### Publishing Events
- **PublishedLive** - 댓글이 라이브 구독자에게 게시됨(구독자 수 포함)

### Integration Events
- **WebhookSynced** - 댓글이 웹훅을 통해 동기화됨

### Spam Rule Events
- **SpamRuleMatch** - 댓글이 맞춤형 스팸 규칙과 일치함(규칙 세부사항 포함)

### Localization Events
- **LocaleDetectedFromText** - 댓글 텍스트에서 언어 로케일이 자동으로 감지됨(감지된 언어 및 로케일 포함)

## 댓글 로그의 사용 사례

댓글 로그는 각 댓글과 함께 자동으로 생성되어 저장됩니다. 다음과 같은 귀중한 통찰을 제공합니다:

- **중재 결정 이해** - 댓글이 왜 승인되었는지, 검토를 위해 보류되었는지, 또는 스팸으로 표시되었는지를 정확히 확인할 수 있습니다
- **승인/스팸 문제 디버깅** - 댓글이 예상대로 동작하지 않을 때 결정 로직을 추적할 수 있습니다
- **사용자 행동 패턴 추적** - 신뢰도 변화와 인증 상태를 모니터링합니다
- **중재자(모더레이터) 조치 감사** - 특정 댓글에 대해 중재자가 수행한 조치를 검토합니다
- **스팸 필터 효율성 조사** - 어떤 감지 엔진이 스팸을 잡아내는지, 어떤 엔진이 잡지 못하는지 확인합니다
- **통합 문제 해결** - 웹훅 동기화 및 알림 전달을 확인합니다

이러한 로그는 중재 프로세스의 투명성을 유지하고 댓글 시스템 동작을 미세 조정하는 데 도움이 됩니다.