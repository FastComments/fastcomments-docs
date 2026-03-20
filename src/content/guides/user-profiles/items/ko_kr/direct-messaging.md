Direct Messaging (DM)은 FastComments 사용자가 공개 댓글 및 프로필 상호작용과는 별개로 서로 1:1 비공개 대화를 나눌 수 있게 해줍니다.

### What is Direct Messaging?

Direct Messaging은 다음을 제공합니다:
- **두 사용자 간의 비공개 대화**
- **알림이 있는 실시간 메시징**
- **세션 간에 유지되는 대화 기록**
- **사용 가능 여부를 표시하는 온라인 상태 표시**
- **새 메시지를 파악할 수 있는 읽지 않은 메시지 추적**

### Starting a Direct Message

DM 대화를 시작하는 방법은 여러 가지가 있습니다:

**From a User Profile:**
1. 사용자의 프로필 페이지로 이동
2. "Direct Messages" 탭 또는 "Send Message" 버튼 클릭
3. 메시지를 입력하고 전송

**From a Comment:**
일부 구현에서는 사용자의 이름이나 아바타를 클릭하여 빠르게 프로필 및 메시징 옵션에 접근할 수 있습니다.

**Direct Link:**
대화 ID나 DM 리다이렉트 링크가 있는 경우 대화에 직접 접근할 수 있습니다.

### Accessing Your Messages

모든 다이렉트 메시지 대화를 보려면:

1. 프로필 페이지로 이동
2. "Direct Messages" 탭 클릭
3. 모든 대화 목록 확인

각 대화에는 다음이 표시됩니다:
- 상대방의 아바타와 이름
- 온라인/오프라인 상태 (온라인일 때 녹색 표시)
- 마지막 메시지 미리보기
- 읽지 않은 메시지 수(있을 경우)
- 마지막 활동 타임스탬프

### Conversation View

대화를 열면 다음을 볼 수 있습니다:

- **전체 메시지 기록** - 본인과 상대방 간의 모든 메시지
- **실시간 업데이트** - 새로운 메시지는 WebSocket을 통해 즉시 표시됩니다
- **온라인 상태** - 상대방이 현재 온라인인지 확인 가능
- **메시지 타임스탬프** - 각 메시지가 전송된 시간
- **메시지 작성** - 새 메시지를 입력하고 전송할 수 있는 텍스트 상자

### Message Notifications

새 다이렉트 메시지에 대한 알림을 받습니다:

- **앱 내 알림** - 프로필의 배지 카운터
- **이메일 알림** - 알림 설정에 따라
- **실시간 알림** - 로그인 상태일 때 즉시 알림

알림 환경설정은 [Account Settings](https://fastcomments.com/auth/my-account/edit-notifications)에서 관리하세요.

### Privacy and Blocking

**Disabling Direct Messages:**
DM 수신을 원하지 않는 경우 완전히 비활성화할 수 있습니다:
1. 프로필의 개인정보 설정으로 이동
2. "Disable Direct Messages" 활성화
3. 프로필에서 DM 옵션이 숨겨집니다

자세한 내용은 [Privacy Settings](/guide-user-profiles.html#privacy-settings)를 참조하세요.

**Blocking Users:**
누군가 DM으로 괴롭힘을 하는 경우:
1. 해당 사용자의 프로필 방문
2. 사용자를 차단
3. 해당 사용자는 더 이상 메시지를 보낼 수 없습니다

차단된 사용자는 다음을 할 수 없습니다:
- 다이렉트 메시지 전송
- 귀하의 활동 보기
- 프로필에 댓글 달기
- FastComments 내에서 귀하와 상호작용

### Managing Conversations

**Hiding Conversations:**
더 이상 보고 싶지 않은 대화를 대화 목록에서 숨길 수 있습니다:
1. Direct Messages 탭 열기
2. 숨기려는 대화 찾기
3. 숨기기/보관 옵션 선택

숨긴 대화는 기본 목록에 나타나지 않지만 상대방이 새 메시지를 보내면 접근할 수 있습니다.

**Marking as Read:**
메시지는 열람하면 자동으로 읽음 처리됩니다. 전체 대화를 수동으로 읽음으로 표시할 수도 있습니다:
1. Direct Messages 탭 열기
2. 대화 선택
3. "Mark as Read" 선택

이렇게 하면 해당 대화의 읽지 않은 카운터가 지워집니다.

### Best Practices

**When to Use Direct Messages:**
- 후속 질문을 비공개로 묻고 싶을 때
- 도움이 된 조언에 대해 감사 인사를 전할 때
- 공개 댓글을 어지럽히지 않고 주제에서 벗어난 대화를 할 때
- 다른 커뮤니티 구성원과 조정할 때
- 개인적인 피드백이나 제안을 제공할 때

**DM Etiquette:**
- 정중하고 전문적으로 행동하세요
- 원치 않는 메시지로 사용자를 스팸하지 마세요
- 누군가 응답하지 않거나 DM을 비활성화하면 이를 존중하세요
- 대화를 관련성 있고 건설적으로 유지하세요
- 허락 없이 다른 사람의 DM을 공유하지 마세요

**Safety Tips:**
- 상대를 신뢰하지 않는 한 개인 정보(전화번호, 주소 등)를 공유하지 마세요
- DM으로 괴롭힘이나 학대를 당하면 차단 및 신고하세요
- 의심스러운 링크나 요청에 주의하세요
- 불편함을 느끼면 차단 기능을 사용하세요

### Limitations and Notes

**Who Can You Message:**
- DM을 비활성화하지 않은 모든 FastComments 사용자
- 귀하를 차단하지 않은 사용자
- 모든 FastComments 커뮤니티의 사용자

**Message Content:**
- 텍스트 메시지를 지원합니다
- 메시지는 댓글과 동일한 콘텐츠 정책을 따릅니다
- 부적절한 콘텐츠는 신고할 수 있습니다

**Conversation Scope:**
- DM 대화는 두 사람 간의 비공개 대화입니다
- 그룹 메시징 없음(현재 1:1 전용)
- 대화 기록은 무기한 보존됩니다

### Troubleshooting

**Can't Send a Message?**
상대방은 다음을 했을 수 있습니다:
- 개인정보 설정에서 다이렉트 메시지를 비활성화함
- 귀하를 차단함
- 계정을 삭제함

**Not Receiving Notifications?**
[Account Settings](https://fastcomments.com/auth/my-account/edit-notifications)에서 DM 알림이 활성화되어 있는지 확인하세요.

**Messages Not Sending?**
- 인터넷 연결을 확인하세요
- 페이지를 새로고침한 후 다시 시도하세요
- 차단되지 않았는지 확인하세요
- 문제가 지속되면 지원팀에 문의하세요