이 가이드는 사이트 관리자가 도구를 등록하고 활동 선택기에서 보이도록 설정한 이후 Moodle 4.x 강좌에 FastComments를 추가하는 방법을 다룹니다. FastComments가 아직 등록되지 않았다면 먼저 Moodle 등록 가이드를 참조하세요.

#### 강좌를 편집 모드로 열기

1. 강좌에 대해 편집 권한이 있는 교사(또는 그 이상) 계정으로 Moodle에 로그인합니다.
2. 강좌를 엽니다.
3. 강좌 헤더 오른쪽 상단의 스위치를 사용하여 **편집 모드**를 켭니다.

Moodle 4.x는 3.x에서 사용하던 레거시 "활동 또는 자원 추가" 드롭다운을 전체 화면 활동 선택기 대화상자로 대체했습니다. Moodle 4.5는 동일한 선택기를 유지하지만 상단에 즐겨찾기 행을 추가하므로 FastComments를 한 번 고정해두면 이후 섹션에서 더 빠르게 접근할 수 있습니다.

#### FastComments 활동 추가

1. 토론을 넣을 강좌 섹션(주제 또는 주간)으로 스크롤합니다.
2. 해당 섹션 하단에서 **활동 또는 자원 추가**를 클릭합니다.
3. 선택기 대화상자에서 **FastComments**를 선택합니다. 보이지 않으면 아래의 주의사항 섹션으로 이동하세요.

활동 설정 양식이 열립니다. 중요한 필드는 다음과 같습니다:

- **활동 이름**(필수). 강좌 페이지와 성적표에 표시됩니다. 예: `Week 3 Discussion`.
- **활동 설명**. 댓글 스레드 위에 렌더링되는 선택적 소개 텍스트입니다.
- **활동 페이지에 설명 표시**. 활동을 클릭하지 않아도 설명을 보이게 하려면 선택합니다.
- **사전 구성된 도구(Preconfigured tool)**. `FastComments`로 설정됩니다(선택기에서 시작하면 자동 선택됨). 변경하지 마세요.
- **런치 컨테이너(Launch container)**. **새 창(New window)**으로 설정하세요. 일부 Moodle 배포에서 "같은 창(Same window)"으로 설정하면 작동하지 않는 이유는 주의사항 섹션을 참고하세요.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. 비워 둡니다. 동적 등록(Dynamic Registration)이 사이트 레벨에서 이를 처리합니다.

페이지 하단으로 스크롤하여 **저장 후 강좌로 돌아가기**(또는 바로 활동을 열려면 **저장 후 표시**)를 클릭합니다.

활동은 FastComments 아이콘과 함께 섹션에 행으로 표시됩니다. 학생들은 해당 행을 클릭하여 댓글 스레드를 엽니다.

#### 편집기에서 FastComments를 인라인으로 삽입

Page, Book 챕터, Lesson 또는 Atto나 TinyMCE 편집기를 사용하는 기타 리소스 안에 스레드를 삽입하려면:

1. 리소스를 편집 모드로 엽니다.
2. 스레드를 넣을 위치에 커서를 놓습니다.
3. 편집기 도구 모음에서 **LTI** / **외부 도구(External tool)** 버튼을 클릭합니다. Atto에서는 "Insert LTI Advantage content"로 표시되고, TinyMCE(기본값은 Moodle 4.3+)에서는 **더보기(More)** 메뉴 아래의 **외부 도구(External tools)**에 있습니다.
4. 도구 목록에서 **FastComments**를 선택합니다.
5. FastComments가 딥링크 선택기를 엽니다. 스레드 제목을 확인하고 **삽입(Embed)**을 클릭합니다.
6. 편집기는 LTI 플레이스홀더 블록을 삽입합니다. 리소스를 저장합니다.

각 삽입 인스턴스는 딥링크 콘텐츠 항목 ID를 키로 하는 개별 스레드이므로, 한 Page에 FastComments를 세 번 삽입하면 세 개의 독립적인 스레드가 생성됩니다.

#### 접근 제한 및 그룹 설정

표준 Moodle 활동 설정이 FastComments 활동에 적용됩니다:

- **공통 모듈 설정(Common module settings)** > **그룹 모드(Group mode)**. 이를 **별도 그룹(Separate groups)** 또는 **보이는 그룹(Visible groups)**으로 설정해도 FastComments가 자동으로 그룹별 스레드로 분리되지는 않습니다. Moodle의 그룹 모드는 성적표와 구성원 목록을 필터링할 뿐입니다. 그룹별로 별도의 스레드를 운영하려면 그룹마다 하나의 FastComments 활동을 추가하고 **접근 제한(Restrict access)**을 사용하여 각 활동의 범위를 지정하세요.
- **접근 제한(Restrict access)** > **제한 추가(Add restriction)**. 표준 Moodle 조건을 지원합니다: **날짜(Date)**, **성적(Grade)**, **그룹(Group)**, **그룹화(Grouping)**, **사용자 프로필(User profile)** 및 중첩 제한 세트. 단일 그룹에 FastComments 활동을 잠그려면 **그룹(Group)**을 사용하세요.
- **활동 완료(Activity completion)**. 완료 추적을 원하면 **학생은 이 활동을 보아야 완료됨(Students must view this activity to complete it)**으로 설정하세요. 현재 FastComments는 런치 이후 Moodle로 완료 이벤트를 반환하지 않습니다.

#### 역할 매핑

FastComments는 Moodle이 매 런치 시 보내는 LTI `roles` 클레임을 읽고 다음과 같이 매핑합니다:

- Moodle **관리자(Manager)** 또는 **사이트 관리자(Site administrator)** -> FastComments **admin**
- Moodle **편집 교사(Editing teacher)** 또는 **비편집 교사(Non-editing teacher)** -> FastComments **moderator**
- Moodle **학생(Student)** -> FastComments **commenter**
- Moodle **게스트(Guest)** -> 읽기 전용

관리자(admin)는 모든 댓글을 삭제하고 사용자를 차단하며 스레드 설정을 편집할 수 있습니다. 중재자(moderator)는 자신이 런치한 스레드 내부에서 댓글을 삭제하고 승인할 수 있습니다. 커스텀 Moodle 역할은 복제한 원형(archetype)의 매핑을 상속합니다.

#### 학생이 보는 내용

학생들은 FastComments 활동을 클릭하거나 Page나 Book 내부의 삽입 블록으로 스크롤합니다. Moodle은 LTI 런치를 통해 그들의 신원을 FastComments로 전송합니다:

- 로그인 화면이 없습니다. FastComments는 Moodle 계정을 사용해 자동으로 로그인합니다.
- 표시 이름, 이메일 및 아바타는 Moodle에서 가져옵니다.
- 스레드는 `(Moodle site, course, resource link ID)`로 범위가 지정되므로 동일한 활동을 다른 강좌에 복제하면 새로운 스레드가 생성됩니다.
- 스레드형 답글, 투표 및 알림은 독립 실행형 FastComments 스레드와 동일하게 작동합니다.

#### Moodle 주의 사항

**활동 선택기에서 FastComments가 보이지 않습니다.** 사이트 관리자가 도구를 등록했지만 **도구 구성 사용(Tool configuration usage)**을 **활동 선택기 및 사전 구성 도구로 표시(Show in activity chooser and as a preconfigured tool)**로 설정하지 않았습니다. 이것은 **사이트 관리(Site administration)** > **플러그인(Plugins)** > **활동 모듈(Activity modules)** > **외부 도구(External tool)** > **도구 관리(Manage tools)** > FastComments 타일의 기어 아이콘에서 수정할 수 있습니다.

**"같은 창(Same window)"으로 설정하면 런치가 실패하거나 빈 프레임이 표시됩니다.** Moodle의 세션 쿠키는 기본적으로 `SameSite=Lax`를 사용하며, 일부 브라우저는 LTI 1.3이 FastComments에서 반환할 때 사용하는 교차 사이트 POST에서 쿠키를 제거합니다. 활동에서 **런치 컨테이너(Launch container)**를 **새 창(New window)**으로 설정하세요. 편집기에 삽입된 FastComments의 경우 편집기에 삽입된 런치 경로는 항상 새 창을 팝업하므로 이것은 필수 조건입니다.

**`iss` 클레임은 테넌트 ID가 아닌 Moodle 사이트 URL입니다.** FastComments는 LTI 발행자(issuer)로 Moodle 사이트 URL(구성값 `wwwroot`)을 사용합니다. Moodle 인스턴스의 도메인이 변경되거나 `wwwroot`를 변경하면 기존 FastComments 스레드는 이전 발행자에 묶인 상태로 남아 새 런치와 일치하지 않습니다. 필요한 경우 도구를 새 URL에 대해 다시 등록하고 FastComments 관리자에서 스레드를 마이그레이션하세요.

**활동 백업 및 복원.** 강좌를 백업하여 새 강좌로 복원하면 새로운 리소스 링크 ID가 생성되므로 복원된 FastComments 활동은 빈 스레드로 시작합니다. 원래 강좌는 원래 스레드를 유지합니다. 이는 의도된 동작이며 버그가 아닙니다.

**Moodle 4.5의 TinyMCE 기본값.** Moodle 4.5는 새로운 설치에 대해 TinyMCE를 기본 편집기로 제공합니다. 외부 도구 버튼은 기본 도구 모음이 아니라 **더보기(...)** 메뉴 아래에 있습니다. 4.1에서 업그레이드한 구형 사이트는 관리자가 기본값을 변경하지 않는 한 Atto를 유지합니다.