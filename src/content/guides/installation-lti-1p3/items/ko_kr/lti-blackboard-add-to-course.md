Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View is the default in Blackboard Learn SaaS as of 2026.

1. 강좌를 열고 **Course Content** 페이지로 이동합니다.
2. 개요에서 댓글 스레드를 배치하고 싶은 위치에 마우스를 올리거나 탭한 다음 보라색의 **+** (Add content) 버튼을 클릭합니다.
3. **Content Market**을 선택합니다. Content Market 패널에는 기관에서 승인한 모든 LTI 도구 및 Building Block 배치가 나열됩니다.
4. **FastComments** 타일을 찾아 클릭합니다. Blackboard는 **+** 메뉴를 연 위치에 콘텐츠 항목을 생성합니다.
5. 이 항목은 기본적으로 개요에 "Visible to students" 항목으로 생성됩니다(강사가 개인 기본 설정으로 **Hide from students**를 끈 경우). 기본값이 **Hidden**이면 항목이 숨김 상태로 생성되며, 준비가 되면 항목 행의 가시성 선택기를 켜서 표시할 수 있습니다.
6. 항목 이름을 변경하려면 개요에서 제목을 클릭하고 새 레이블을 입력합니다. 학생이 개요에서 보는 제목은 FastComments 스레드 식별자와 독립적이므로 언제든지 이름을 변경해도 안전합니다.

**Content Market** 옵션이 보이지 않는 경우, 귀 기관에서 해당 배치를 숨겨둔 것입니다. 동일한 선택기는 같은 **+** 메뉴의 **LTI Tools** 그룹 아래 **More tools**를 통해서도 접근할 수 있습니다.

#### Original Course View

Original Course View는 Learn SaaS에서 계속 지원되며 Q4 2024 CU 릴리스 라인의 자체 호스팅된 Learn 9.1 사이트에서는 여전히 기본 환경입니다.

1. 강좌를 열고 **Content Area**(예: 강좌 메뉴의 기본 **Information** 또는 **Content** 영역)에 들어갑니다.
2. 페이지 오른쪽 상단의 토글로 **Edit Mode**를 켭니다.
3. 작업 표시줄에서 **Build Content**를 클릭합니다.
4. **Learning Tools** 하위 메뉴에서 **FastComments**를 클릭합니다. Learning Tools 하위 메뉴는 관리자가 도구를 등록한 후 LTI 1.3 도구 배치로 채워집니다. 보이지 않으면 아래의 문제 해결 섹션을 참조하십시오.
5. **Create FastComments** 양식에서 다음을 설정합니다:
   - **Name**: 학생이 콘텐츠 영역에서 볼 레이블.
   - **Description**: 임베드된 스레드 위에 표시되는 선택적 텍스트.
   - **Permit Users to View this Content**: 사용 가능 여부 토글(예/아니오).
   - **Track Number of Views**: Blackboard의 항목별 조회 통계를 원하면 활성화합니다. FastComments는 자체 분석을 별도로 실행합니다.
   - **Date and Time Restrictions**: 선택적 **Display After** / **Display Until** 기간.
6. 제출합니다. 도구가 콘텐츠 영역에 클릭 가능한 항목으로 나타납니다.

#### Embedding Inside an Item or Document

두 강좌 보기 모두에서 강사는 Content Editor의 LTI Advantage 버튼을 통해 Item, Document 또는 모든 리치 텍스트 필드 본문 안에 FastComments를 인라인으로 임베드합니다.

Ultra Course View:

1. **Document**를 만들거나 편집합니다.
2. 스레드를 표시하고 싶은 문서 본문 안에서 **Add content**를 클릭합니다.
3. 편집기 도구 모음에서 **Insert content** 메뉴를 열고 **Content Market**(LTI Advantage / Deep Linking 진입점)을 클릭합니다.
4. **FastComments**를 선택합니다. FastComments는 딥링크 페이로드를 반환하고 Blackboard는 커서 위치에 임베디드 블록을 문서 본문에 삽입합니다.
5. 문서를 저장합니다. 학생은 스크롤하여 해당 부분을 지나갈 때 스레드가 인라인으로 렌더링된 것을 봅니다.

Original Course View:

1. 리치 텍스트 본문이 있는 항목을 편집합니다.
2. Content Editor 도구 모음에서 **Add Content** 더하기 아이콘을 클릭하고 **Content Market**을 선택합니다(구형 Q4 2024 CU에서는 **Add Content from External Tool**로 표시됨).
3. **FastComments**를 선택합니다. 편집기는 딥링크된 리소스를 참조하는 플레이스홀더 블록을 삽입합니다.
4. 항목을 제출합니다.

각 딥링크 임베드는 자체 FastComments 스레드를 생성하므로, 한 Item에 두 개의 FastComments 블록이 임베드되어 있으면 두 개의 독립된 댓글 스트림이 생성됩니다.

#### Visibility, Release Conditions, and Group Restrictions

FastComments 콘텐츠 항목은 그 위에 적용되는 접근 제어 규칙에 대해 다른 Blackboard 콘텐츠 항목과 동일하게 동작합니다.

- Ultra: 행의 가시성 선택기(**Visible to students**, **Hidden from students**, **Conditional availability**)를 클릭합니다. Conditional availability는 날짜/시간 창, 성적부 항목에 대한 성과 규칙, 강좌 그룹에 대한 구성원 규칙을 지원합니다.
- Original: 항목의 컨텍스트 메뉴를 열고 **Adaptive Release** 또는 **Adaptive Release: Advanced**를 선택하여 날짜, 구성원, 성적 또는 검토 상태로 도구를 제한합니다. 항목의 **Set Group Availability**를 사용하여 특정 강좌 그룹으로 제한합니다.

FastComments는 Blackboard의 게이트에서 결정한 것을 따릅니다. Blackboard가 학생에게 항목을 숨기면 해당 학생에 대해서는 LTI 런치가 전혀 발생하지 않으며 그 학생은 moderator 뷰에 나타나지 않습니다.

#### Gradebook Behavior

FastComments는 LTI Advantage Assignment and Grade Services를 통해 성적을 보고하지 않습니다. FastComments 콘텐츠 항목에 대해 자동으로 성적 열이 생성되지 않습니다.

기관의 Blackboard 테넌트가 채점 메타데이터와 관계없이 새 콘텐츠 항목마다 성적부 열을 자동 생성하도록 구성된 경우, 빈 열이 그래도 나타날 수 있습니다. 이를 숨기려면:

- Ultra: **Gradebook**을 열고 열 헤더를 클릭한 다음 **Edit**를 선택하고 **Show to students** 및 **Include in calculations**를 끕니다. 또는 기관에서 비채점 항목에 대한 열 삭제를 허용하면 **Delete**를 사용합니다.
- Original: **Grade Center**를 열고 열의 첼러번(chevron)을 클릭한 다음 **Hide from Users (on/off)**를 선택하고 선택적으로 **Column Organization** 아래에서 **Hide from Instructor View**를 선택합니다.

#### What Students See

학생이 FastComments 항목을 열거나 임베드된 블록으로 스크롤하면:

1. Blackboard가 FastComments로 LTI 1.3 메시지를 런치합니다. 학생은 Blackboard 신원(이름, 이메일, 아바타, 역할)을 사용하여 SSO로 로그인되며 로그인 양식을 보지 않습니다.
2. 댓글 스레드가 iframe에서 렌더링됩니다. 스레딩, 답글, 멘션 및 리액션은 FastComments에서 구성된 댓글 위젯 설정에 따라 모두 사용 가능합니다.
3. 학생의 댓글은 그들의 Blackboard 계정에 귀속됩니다. 학생이 나중에 Blackboard에서 이름이나 사진을 편집하면 다음 런치 시 FastComments 프로필이 업데이트됩니다.

Blackboard에서 FastComments로의 역할 매핑:

- **System Administrator** 및 **Course Builder**는 FastComments의 **admin**으로 매핑됩니다.
- **Instructor** 및 **Teaching Assistant**는 FastComments의 **moderator**로 매핑됩니다.
- **Student**, **Guest**, 및 **Observer**는 FastComments의 **commenter**로 매핑됩니다.

Moderator는 스레드의 모든 댓글 옆에 표시되는 핀(pin), 숨기기(hide), 차단(ban), 삭제(delete) 같은 중재 컨트롤을 보게 됩니다.

#### Thread Scoping

FastComments는 각 스레드를 **(Blackboard host, course ID, resource link ID)**로 범위를 한정합니다. 동일한 강좌 내의 두 FastComments 항목은 두 개의 스레드를 생성합니다. 동일한 항목이 두 개의 강좌 쉘에 복사된 경우(예: 강좌 복사를 통해) Blackboard가 복사 중에 새로운 resource link ID를 발급하므로 두 개의 스레드가 생성됩니다. 강좌 복사 시에도 스레드를 공유하려면, 복사하기 전에 FastComments에서 명시적 스레드 URN을 구성한 Deep Linking을 사용하십시오.

#### Blackboard-Specific Gotchas

**FastComments 타일이 Build Content 메뉴(Original) 또는 Content Market(Ultra)에 없음.** 관리자가 도구를 승인했지만 관련 배치를 차단하는 기관 정책을 남겨두었습니다. **Administrator Panel** > **Integrations** > **LTI Tool Providers**로 이동하여 FastComments 항목을 편집하고 **Course Content Tool**(Original) 및 **Course Content Tool - allow students** / **Deep Linking content tool**(Ultra) 배치가 모두 활성화되어 있는지 확인하십시오. 저장한 후 강좌 페이지를 새로고침합니다.

**런치 시 "Tool not configured for this context" 또는 "Tool is not deployed" 오류.** 동적 등록 중에 등록된 배포 범위가 강좌가 속한 기관 컨텍스트와 일치하지 않습니다. Blackboard의 도구 제공자 항목에서 **Deployment ID**가 이 테넌트의 FastComments LTI 1.3 Configuration 페이지에 표시된 값과 일치하는지 확인하십시오. 다르면 배치를 삭제하고 새 등록 URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기에서 가져오십시오</a>)에서 동적 등록을 다시 실행하십시오.

**Iframe 높이가 고정되어 보이거나 내용이 잘림.** 일부 Blackboard 테넌트는 기본 LTI iframe-resize postMessage를 차단하는 엄격한 Content Security Policy를 적용합니다. FastComments는 호환성을 극대화하기 위해 Canvas 스타일의 `lti.frameResize` 메시지와 IMS 규격형 `org.imsglobal.lti.frameResize` 메시지 둘 다를 발신하지만, 테넌트 수준의 CSP 오버라이드가 부모 리스너를 차단합니다. 관리자에게 `*.fastcomments.com`이 LTI 도구 허용 목록에 포함되어 있는지와 맞춤 CSP 헤더가 postMessage 이벤트를 제거하고 있지 않은지 확인해 달라고 요청하십시오. 그러면 추가 구성 없이 리사이즈가 작동합니다.

**Course copy가 스레드를 중복 생성함.** Blackboard의 강좌 복사는 LTI 배치에 대해 새로운 resource link ID를 발급하므로 복사된 강좌는 빈 스레드로 시작합니다. 이는 예상된 동작입니다. 복사된 강좌가 원본 스레드를 상속받아야 하는 경우에는 복사 전에 명시적 스레드 URN으로 Deep Linking을 설정하거나 FastComments 지원팀에 문의하여 대량으로 스레드 ID를 재매핑하십시오.

**학생이 런치 시 일반적인 Blackboard 오류를 봄.** 원인은 누락되었거나 오래된 `email` 클레임일 수 있습니다. FastComments에 대한 기관 정책에서 **User Fields to Send** 아래에 **Role**, **Name**, **Email Address**가 활성화되어 있는지 확인하십시오. 저장한 다음 새 브라우저 세션에서 다시 런치해 보십시오.