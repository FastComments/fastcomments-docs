FastComments가 귀하의 LMS에 등록되면 강사는 다른 LTI 외부 도구를 추가하는 것과 같은 방법으로 코스에 FastComments를 추가합니다.

#### D2L Brightspace

코스의 콘텐츠 영역에서:

1. **Add Existing Activities** > **External Learning Tools**를 클릭합니다.
2. 목록에서 **FastComments**를 선택합니다.
3. 해당 도구가 콘텐츠 영역에 토픽으로 나타납니다. 해당 리소스의 댓글 스레드를 초기화하려면 한 번 열어봅니다.

#### Moodle

코스에서:

1. **Edit mode**를 켭니다.
2. 댓글을 추가하려는 섹션에서 **Add an activity or resource**를 클릭합니다.
3. 활동 선택기에서 **FastComments**를 선택합니다.
4. 저장합니다. 학생들은 섹션에 임베드된 댓글 스레드를 보게 됩니다.

#### Blackboard Learn

코스에서:

1. 콘텐츠 영역으로 이동합니다.
2. **Build Content** > **FastComments**(“Learning Tools” 아래)를 클릭합니다.
3. 이름을 구성하고 제출합니다.

#### Sakai

사이트 관리자는 **Site Info** > **Manage Tools** > **External Tools**까지 스크롤하여 **FastComments**를 선택한 다음 **Continue**를 통해 도구를 추가합니다.

#### How Threads Are Scoped

FastComments는 **(LMS instance, course, resource link)**별로 별도의 댓글 스레드를 생성합니다. 즉:

- 같은 LMS의 서로 다른 코스 두 개는 도구 이름이 같더라도 별도의 스레드를 가집니다.
- 하나의 코스 내에서 동일한 FastComments 도구를 두 곳에 사용하면 두 개의 스레드가 생성됩니다.
- 동일한 FastComments 계정에 속한 서로 다른 두 Brightspace 설치는 별개의 스레드를 갖습니다 - LMS 호스트네임이 스레드 식별자의 일부입니다.

#### SSO

학생들은 로그인 화면을 보지 않습니다. LMS는 LTI 런치를 통해 그들의 신원(이름, 이메일, 아바타, 역할)을 FastComments로 전송하며, FastComments는 자동으로 로그인 처리합니다. 그들의 댓글은 LMS 계정에 귀속됩니다.

LMS에서 **Instructor** 또는 **Administrator** 역할을 가진 사용자는 해당 스레드에서 FastComments의 중재자/관리자 역할로 자동 매핑됩니다.