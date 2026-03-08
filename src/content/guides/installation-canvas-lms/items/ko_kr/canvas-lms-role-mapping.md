Canvas 역할은 LTI 시작 시 FastComments 역할에 자동으로 매핑됩니다. 수동 구성은 필요하지 않습니다.

#### 역할 매핑

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | 전체 계정 접근, 모든 댓글 및 설정 관리 |
| **Instructor** | Moderator | 댓글 편집 및 삭제, 스레드 고정, 토론 관리 |
| **Learner** | Commenter | 댓글 작성, 답글, 투표 및 멘션 사용 |

#### 작동 방식

사용자가 Canvas에서 FastComments를 실행하면 LTI 1.3 프로토콜에 해당 사용자의 Canvas 역할이 포함됩니다. FastComments는 이 역할을 읽어 적절한 권한을 자동으로 할당합니다.

If a user has multiple roles (e.g. an Instructor who is also an Admin), the highest-privilege role is used.

---