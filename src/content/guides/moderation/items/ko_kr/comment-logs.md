FastComments는 각 댓글에 대한 자세한 이벤트를 자동으로 추적하여 중재 결정 및 시스템 작업에 대한 투명성을 제공합니다. 이러한 로그를 통해 댓글이 승인되었는지, 스팸으로 표시되었는지, 또는 상태가 변경된 이유를 이해할 수 있습니다.

## 댓글 로그에 접근하기

1. FastComments 대시보드에서 **Moderate Comments** 페이지로 이동합니다  
2. 검토하려는 댓글을 찾습니다  
3. 댓글 작업 표시줄에서 **View Logs** 버튼(시계 아이콘)을 클릭합니다  
4. 해당 댓글에 대한 전체 이벤트 기록을 보여주는 대화 상자가 나타납니다  

각 로그 항목은 다음을 표시합니다:
- **When** - 이벤트의 타임스탬프  
- **Who** - 이벤트를 트리거한 사용자 또는 시스템(해당되는 경우)  
- **What** - 작업 또는 이벤트 유형  
- **Details** - 이전/이후 값, 엔진 이름 또는 관련 데이터와 같은 추가 컨텍스트  

## 댓글 로그 이벤트

각 댓글은 수명 주기 동안 발생하는 이벤트 로그를 유지합니다. 아래는 추적되는 이벤트 유형입니다:

### 익명화 이벤트
- **Anonymized** - 댓글 내용이 삭제되고 사용자가 삭제된 것으로 표시됨  
- **RestoredFromAnonymized** - 익명화된 상태에서 댓글이 복원됨  

### 승인 이벤트
- **ApprovedDueToPastComment** - 사용자가 이전에 댓글을 승인한 적이 있어 댓글이 승인됨(과거 댓글에 대한 참조 포함)  
- **ApprovedIsAdmin** - 사용자가 관리자이므로 댓글이 승인됨  
- **NotApprovedRequiresApproval** - 댓글에 수동 승인이 필요함  
- **NotApprovedLowTrustFactor** - 사용자 신뢰도가 낮아 댓글이 승인되지 않음(신뢰도 값 포함)  

### 프로필 댓글 승인 이벤트
이 이벤트는 사용자 프로필에 대한 댓글에만 적용됩니다:
- **ApprovedProfileAutoApproveAll** - 프로필 소유자가 모든 댓글에 대해 자동 승인을 활성화했기 때문에 프로필 댓글이 자동 승인됨  
- **ApprovedProfileTrusted** - 댓글 작성자가 신뢰할 수 있어 프로필 댓글이 승인됨(신뢰를 형성한 댓글에 대한 참조 포함)  
- **NotApprovedProfileManualApproveAll** - 프로필 소유자가 수동 승인을 활성화했기 때문에 프로필 댓글에 수동 승인이 필요함  
- **NotApprovedProfileNotTrusted** - 댓글 작성자가 신뢰되지 않아 프로필 댓글이 승인되지 않음  
- **NotApprovedProfileNewUser** - 댓글 작성자가 신규 사용자라서 프로필 댓글이 승인되지 않음  

### 스팸 감지 이벤트
- **IsSpam** - 감지 엔진에 의해 스팸으로 표시됨(결정을 내린 엔진 포함)  
- **IsSpamDueToBadWords** - 욕설 필터 때문에 스팸으로 표시됨  
- **IsSpamFromLLM** - AI/LLM 엔진에 의해 스팸으로 표시됨(엔진 이름, 응답 및 토큰 수 포함)  
- **IsSpamRepeatComment** - 반복적인 댓글이라 스팸으로 표시됨(감지한 엔진 포함)  
- **NotSpamIsOnlyImage** - 이미지만 포함되어 있어 스팸으로 표시되지 않음  
- **NotSpamIsOnlyReacts** - 반응만 포함되어 있어 스팸으로 표시되지 않음  
- **NotSpamNoLinkOrMention** - 의심스러운 링크나 멘션이 없어 스팸으로 표시되지 않음  
- **NotSpamPerfectTrustFactor** - 사용자 신뢰도가 높아 스팸으로 표시되지 않음  
- **NotSpamTooShort** - 분석하기에 너무 짧아 스팸으로 표시되지 않음  
- **NotSpamSkipped** - 스팸 검사가 건너뛰어짐  
- **NotSpamFromEngine** - 감지 엔진에 의해 스팸이 아닌 것으로 판단됨(엔진 이름 및 신뢰도 포함)  

### 욕설/부적절 언어 이벤트
- **BadWordsCheckFailed** - 욕설 필터 검사 중 오류 발생  
- **BadWordsFoundBadPhrase** - 욕설 필터가 부적절한 구문을 감지함(구문 포함)  
- **BadWordsFoundBadWord** - 욕설 필터가 부적절한 단어를 감지함(단어 포함)  
- **BadWordsNoDefinitionForLocale** - 댓글 언어에 대한 욕설 정의가 없음(로케일 포함)  

### 사용자 인증 이벤트
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** - 댓글에 인증이 필요하지만 사용자가 인증된 세션에 없음  
- **CommentMustBeVerifiedToApproveNotVerifiedYet** - 댓글에 인증이 필요하지만 사용자가 아직 인증되지 않음  
- **InVerifiedSession** - 댓글을 게시하는 사용자가 인증된 세션에 있음  
- **SentVerificationEmailNoSession** - 인증되지 않은 사용자에게 인증 이메일 전송  
- **SentWelcomeEmail** - 신규 사용자에게 환영 이메일 전송  

### 신뢰 및 보안 이벤트
- **TrustFactorChanged** - 사용자의 신뢰도가 변경됨(이전 및 이후 값 포함)  
- **SpamFilterDisabledBecauseAdmin** - 관리자 사용자에 대해 스팸 필터링이 우회됨  
- **TenantSpamFilterDisabled** - 전체 테넌트에 대해 스팸 필터링이 비활성화됨  
- **RepeatCommentCheckIgnored** - 반복 댓글 검사가 우회됨(이유 포함)  
- **UserIsAdmin** - 사용자가 관리자임  
- **UserIsAdminParentTenant** - 사용자가 상위 테넌트 관리자임  
- **UserIsAdminViaSSO** - 사용자가 SSO를 통해 관리자임  
- **UserIsMod** - 사용자가 모더레이터임  

### 댓글 상태 변경
상태 변경 이벤트는 이전 및 이후 값과 변경을 수행한 사용자를 포함합니다:
- **ExpireStatusChanged** - 댓글 만료 상태가 수정됨  
- **ReviewStatusChanged** - 댓글 검토 상태가 변경됨  
- **SpamStatusChanged** - 댓글 스팸 상태가 업데이트됨  
- **ApproveStatusChanged** - 댓글 승인 상태가 변경됨  
- **TextChanged** - 댓글 텍스트 내용이 편집됨(이전 및 이후 텍스트 포함)  
- **VotesChanged** - 댓글 투표 수가 업데이트됨(상세 투표 내역 포함)  
- **Flagged** - 사용자가 댓글을 신고함  
- **UnFlagged** - 댓글 신고가 해제됨  

### 중재 작업
- **Pinned** - 모더레이터가 댓글을 고정함(고정한 사람 포함)  
- **UnPinned** - 모더레이터가 댓글 고정을 해제함(해제한 사람 포함)  

### 알림 이벤트
- **CreatedNotifications** - 댓글에 대한 알림이 생성됨(알림 수 포함)  
- **NotificationCreateFailure** - 알림 생성 실패  
- **BadgeAwarded** - 댓글에 대해 사용자 배지가 부여됨(배지 이름 포함)  

### 멘션 및 답글 알림 이벤트
이 이벤트는 이메일 또는 알림을 받을 사람을 지정합니다. 아무것도 전송되지 않은 경우 Details 열에 이유가 표시됩니다.
- **MentionEmailSent** - 댓글에 멘션된 사용자에게 이메일 전송  
- **MentionEmailSkipped** - 멘션된 사용자에게 이메일이 전송되지 않음(이유 포함)  
- **MentionHeldForApproval** - 멘션 이메일이 댓글 승인될 때까지 대기 중  
- **MentionNotificationCreated** - 멘션된 사용자에게 인앱 알림 생성  
- **MentionNotificationSkipped** - 멘션된 사용자에게 인앱 알림이 전송되지 않음(이유 포함)  
- **ReplyEmailSent** - 답글 대상 댓글 작성자에게 이 답글에 대한 이메일 전송  
- **ReplyEmailSkipped** - 답글 대상 댓글 작성자에게 이메일이 전송되지 않음(이유 포함)  
- **ReplyNotificationSkipped** - 답글 대상 댓글 작성자에게 인앱 알림이 전송되지 않음(이유 포함)  

이메일 또는 알림이 전송되지 않은 경우 표시되는 이유:
- 사용자가 더 이상 존재하지 않거나 이메일 주소가 없음  
- 사용자가 이메일 알림을 끄거나 해당 스레드에 대한 알림을 끔  
- 두 사용자 중 한 명이 다른 사용자를 차단함  
- 사용자가 동일한 SSO 그룹에 속해 있지 않음  
- 사용자의 이메일 주소가 반송 또는 스팸 신고 후 억제 목록에 있음([Email Suppression Management](/guide-notifications.html#email-suppression-management) 참조)  
- 사용자의 이메일 주소가 example.com에 있어 이메일을 받을 수 없음  
- 댓글이 스팸으로 표시되었거나, 삭제되었거나, 7일 이내에 승인되지 않음  
- 답글 대상 댓글이 익명으로 남겨짐  
- 사용자가 자신의 댓글에 답글을 달음  
- 사용자가 답글에 멘션되어 멘션 이메일을 받았고, 답글 이메일은 받지 않음  
- 사용자가 이미 해당 댓글에 대한 답글 알림을 받음  
- 전송이 5회 실패함  

전송이 실패하거나 전송 제한에 도달하면 이메일이 재시도 대기열에 들어가며 로그 항목에 해당 내용이 기록됩니다.

### 게시 이벤트
- **PublishedLive** - 실시간 구독자에게 댓글이 게시됨(구독자 수 포함)  

### 통합 이벤트
- **WebhookSynced** - 웹훅을 통해 댓글이 동기화됨  

### 스팸 규칙 이벤트
- **SpamRuleMatch** - 댓글이 맞춤형 스팸 규칙에 일치함(규칙 세부 정보 포함)  

### 현지화 이벤트
- **LocaleDetectedFromText** - 댓글 텍스트에서 언어 로케일이 자동으로 감지됨(감지된 언어 및 로케일 포함)  

## 댓글 로그 활용 사례

댓글 로그는 각 댓글과 함께 자동으로 생성 및 저장됩니다. 다음과 같은 유용한 인사이트를 제공합니다:
- **Understanding moderation decisions** - 댓글이 승인, 검토 대기, 또는 스팸으로 표시된 정확한 이유 확인  
- **Debugging approval/spam issues** - 댓글이 예상대로 동작하지 않을 때 의사결정 로직을 추적  
- **Tracking user behavior patterns** - 신뢰도 변화 및 인증 상태 모니터링  
- **Auditing moderator actions** - 모더레이터가 특정 댓글에 수행한 작업 검토  
- **Investigating spam filter effectiveness** - 어떤 감지 엔진이 스팸을 잡고 어떤 엔진이 잡지 못하는지 확인  
- **Troubleshooting integrations** - 웹훅 동기화 및 알림 전송 확인  

이러한 로그는 중재 프로세스의 투명성을 유지하고 댓글 시스템 동작을 미세 조정하는 데 도움이 됩니다.