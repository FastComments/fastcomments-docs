이 가이드는 사이트 관리자가 도구를 등록하고 활동 선택기에서 표시되도록 설정한 후 Moodle 4.x 코스에 FastComments를 추가하는 방법을 설명합니다. FastComments가 아직 등록되지 않았다면 먼저 Moodle 등록 가이드를 참조하세요.

#### 강좌를 편집 모드로 열기

1. 해당 강좌에 대해 편집 권한이 있는 강사(또는 그 이상)로 Moodle에 로그인합니다.
2. 강좌를 엽니다.
3. 강좌 헤더 오른쪽 상단의 스위치를 사용하여 **편집 모드**를 켭니다.

Moodle 4.x는 3.x에서 사용하던 레거시 "활동 또는 자원 추가" 드롭다운을 전체 화면 활동 선택기 대화상자로 대체했습니다. Moodle 4.5는 동일한 선택기를 유지하지만 상단에 즐겨찾기/별표 행을 추가하므로 FastComments를 한 번 고정하면 이후 섹션에서 더 빨리 접근할 수 있습니다.

#### FastComments 활동 추가

1. 토론을 배치할 강좌 섹션(주제 또는 주차)으로 스크롤합니다.
2. 해당 섹션 하단의 **활동 또는 자원 추가**를 클릭합니다.
3. 선택기 대화상자에서 **FastComments**를 선택합니다. 보이지 않으면 아래의 주의사항(gotchas) 섹션으로 이동하세요.

활동 설정 폼이 열립니다. 중요한 필드:

- **Activity name** (필수). 강좌 페이지와 성적부에 표시됩니다. 예: `Week 3 Discussion`.
- **Activity description**. 댓글 스레드 위에 렌더링되는 선택적 소개 텍스트입니다.
- **Show description on course page**. 활동을 클릭하지 않고도 설명을 보이게 하려면 체크하세요.
- **Preconfigured tool**. `FastComments`로 설정합니다(선택기에서 실행하면 자동 선택됨). 변경하지 마세요.
- **Launch container**. **New window**로 설정합니다. 일부 Moodle 배포판에서 "Same window"가 문제가 되는 이유는 gotchas 섹션을 참조하세요.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. 비워 둡니다. 동적 등록(Dynamic Registration)이 사이트 수준에서 이를 처리합니다.

다음으로 스크롤하여 **Save and return to course**(또는 활동을 바로 열려면 **Save and display**)를 클릭하세요.

활동은 FastComments 아이콘과 함께 섹션에 행으로 표시됩니다. 학생들은 행을 클릭하여 댓글 스레드를 엽니다.

#### 편집기 내에 FastComments를 인라인으로 삽입

Page, Book 장, Lesson 또는 Atto 또는 TinyMCE 편집기를 사용하는 다른 리소스 안에 스레드를 넣으려면:

1. 리소스를 편집 모드로 엽니다.
2. 스레드를 배치할 위치에 커서를 둡니다.
3. 편집기 툴바에서 **LTI** / **External tool** 버튼을 클릭합니다. Atto에서는 "Insert LTI Advantage content"로 표시됩니다. TinyMCE(기본값은 Moodle 4.3+)에서는 **More** 메뉴 아래의 **External tools**에 있습니다.
4. 도구 목록에서 **FastComments**를 선택합니다.
5. FastComments가 딥링크 선택기를 엽니다. 스레드 제목을 확인하고 **Embed**를 클릭합니다.
6. 편집기는 LTI 자리표시자 블록을 삽입합니다. 리소스를 저장하세요.

각 삽입 인스턴스는 딥링크 콘텐츠 항목 ID를 키로 하는 별개의 스레드이므로, Page에 FastComments 임베드가 세 개 있으면 세 개의 독립된 스레드가 생성됩니다.

#### 접근 제한 및 그룹 설정

표준 Moodle 활동 설정이 FastComments 활동에 적용됩니다:

- **Common module settings** > **Group mode**. 이를 **Separate groups** 또는 **Visible groups**로 설정해도 FastComments가 자동으로 그룹별 스레드로 분리되지는 않습니다. Moodle의 그룹 모드는 성적부와 구성원 목록을 필터링할 뿐입니다. 그룹별로 별도의 스레드를 운영하려면 그룹마다 하나의 FastComments 활동을 추가하고 **Restrict access**를 사용하여 각 활동의 범위를 제한하세요.
- **Restrict access** > **Add restriction**. 표준 Moodle 조건(**Date**, **Grade**, **Group**, **Grouping**, **User profile**, 중첩된 제한 집합)을 지원합니다. FastComments 활동을 단일 그룹에 잠그려면 **Group**을 사용하세요.
- **Activity completion**. 완료 추적을 원한다면 **Students must view this activity to complete it**로 설정하세요. FastComments는 현재 런치 외에는 Moodle에 완료 이벤트를 보고하지 않습니다.

#### 역할 매핑

FastComments는 Moodle이 모든 런치 시 전송하는 LTI `roles` 클레임을 읽어 다음과 같이 매핑합니다:

- Moodle **Manager** 또는 **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** 또는 **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> 읽기 전용

관리자는 모든 댓글을 삭제하고 사용자를 차단하며 스레드 설정을 편집할 수 있습니다. 중재자는 자신이 런치한 스레드 내에서 댓글을 삭제하고 승인할 수 있습니다. 사용자 지정 Moodle 역할은 복제된 원형(archetype)이 상속한 매핑을 따릅니다.

#### 학생들이 보는 것

학생들은 FastComments 활동을 클릭하거나 Page나 Book 안에 삽입된 블록으로 스크롤합니다. Moodle은 LTI 런치를 통해 그들의 정체성을 FastComments로 전송합니다:

- 로그인 화면이 없습니다. FastComments는 Moodle 계정을 사용해 자동으로 로그인시킵니다.
- 표시 이름, 이메일, 아바타는 Moodle에서 가져옵니다.
- 스레드는 `(Moodle site, course, resource link ID)`로 범위가 지정되므로 동일한 활동을 다른 강좌에 복제하면 새로운 스레드가 생성됩니다.
- 스레드형 답글, 투표, 알림은 단독 FastComments 스레드와 동일하게 작동합니다.

#### 공개 접근 제한(권장)

기본적으로 FastComments 댓글 데이터는 공개적으로 읽을 수 있습니다. 스레드의 URL이나 API 엔드포인트를 추측할 수 있는 누구나 Moodle 외부에서도 해당 댓글을 볼 수 있습니다. 강좌 토론의 경우 등록된 학생으로만 보기 권한을 제한하는 것이 거의 항상 필요합니다.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a>를 열고 **Require SSO To View Comments**가 활성화된 규칙을 생성한 다음 보안 수준을 **Secure SSO**로 설정하세요. 그러면 서명된 LTI 런치를 통해서만 스레드를 로드할 수 있습니다.

전체 절차(단일 도메인 또는 페이지에 규칙을 범위 지정하는 방법 포함)는 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)를 참조하세요.

#### Moodle 주의사항 (Gotchas)

**활동 선택기에 FastComments가 없음.** 사이트 관리자가 도구를 등록했지만 **Tool configuration usage**를 **Show in activity chooser and as a preconfigured tool**로 설정하지 않았습니다. 이를 수정하려면 **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > FastComments 타일의 톱니 아이콘으로 이동하세요.

**"Same window"로 설정하면 런치가 실패하거나 빈 프레임이 표시됨.** Moodle의 세션 쿠키는 기본적으로 `SameSite=Lax`를 사용하며 일부 브라우저는 LTI 1.3이 FastComments에서 돌아올 때 사용하는 교차 사이트 POST에서 쿠키를 제거합니다. 활동의 **Launch container**를 **New window**로 설정하세요. 편집기에 임베드된 FastComments의 경우 편집기 삽입 런치 경로가 항상 새 창을 팝업하기 때문에 이는 필수 요구사항입니다.

**`iss` 클레임은 테넌트 ID가 아니라 Moodle 사이트 URL입니다.** FastComments는 LTI 발급자(issuer)로 Moodle 사이트 URL(`wwwroot` 구성 값)을 사용합니다. Moodle 인스턴스가 새 도메인으로 이동하거나 `wwwroot`를 변경하면 기존 FastComments 스레드는 이전 발급자에 연결된 상태로 남아 새 런치와 일치하지 않습니다. 필요하면 새 URL로 도구를 다시 등록하고 FastComments 관리자에서 스레드를 마이그레이션하세요.

**활동 백업 및 복원.** 코스를 백업하여 새 코스로 복원하면 새로운 리소스 링크 ID가 생성되므로 복원된 FastComments 활동은 빈 스레드로 시작합니다. 원래 강좌는 원래 스레드를 유지합니다. 이는 의도된 동작이며 버그가 아닙니다.

**Moodle 4.5의 TinyMCE 기본값.** Moodle 4.5는 새 설치에서 TinyMCE를 기본 편집기로 제공합니다. External tool 버튼 위치는 기본 툴바가 아니라 **More**(`...`) 메뉴 아래에 있습니다. 4.1에서 업그레이드한 오래된 사이트는 관리자가 기본을 변경하지 않았다면 Atto를 계속 사용합니다.