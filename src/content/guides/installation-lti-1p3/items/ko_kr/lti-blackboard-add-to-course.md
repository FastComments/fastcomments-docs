관리자가 FastComments를 LTI 1.3 Advantage 도구로 등록하고 기관 정책을 승인하면, 강사는 표준 Blackboard 배치 포인트를 통해 강좌에 이를 추가합니다. 정확한 절차는 Ultra Course View와 Original Course View에서 다르므로 두 경우를 모두 아래에 설명합니다.

#### Ultra Course View

Ultra Course View는 2026년 기준 Blackboard Learn SaaS의 기본값입니다.

1. 강좌를 열고 **Course Content** 페이지로 이동합니다.
2. 개요에서 댓글 스레드를 배치할 위치에 마우스를 올리거나 탭한 다음 보라색 **+** (콘텐츠 추가) 버튼을 클릭합니다.
3. **Content Market**을 선택합니다. Content Market 패널에는 기관에서 승인한 모든 LTI 도구와 Building Block 배치가 나열됩니다.
4. **FastComments** 타일을 찾아 클릭합니다. Blackboard는 **+** 메뉴를 연 위치에 콘텐츠 항목을 생성합니다.
5. 항목은 기본적으로 개요에 "Visible to students" 항목으로 표시됩니다(개인 기본 설정에서 **Hide from students**가 꺼져 있는 강사용). 기본값이 **Hidden**인 경우 항목이 숨겨진 상태로 생성되며 준비가 되면 항목 행의 가시성 선택기를 전환하여 표시할 수 있습니다.
6. 항목 이름을 바꾸려면 개요에서 제목을 클릭하고 새 레이블을 입력합니다. 학생이 개요에서 보는 제목은 FastComments 스레드 식별자와 독립적이므로 언제든지 이름을 변경해도 안전합니다.

**Content Market**이 옵션으로 보이지 않으면 기관에서 해당 배치를 숨긴 것입니다. 동일한 **+** 메뉴의 **More tools**를 통해 **LTI Tools** 그룹 아래에서 동일한 선택기를 사용할 수도 있습니다.

#### Original Course View

Original Course View는 Learn SaaS에서 계속 지원되며 Q4 2024 CU 릴리스 라인의 자체 호스팅된 Learn 9.1 사이트에 대한 주요 환경으로 남아 있습니다.

1. 강좌를 열고 **Content Area**(예: 강좌 메뉴의 기본 **Information** 또는 **Content** 영역)에 들어갑니다.
2. 페이지 오른쪽 상단의 토글로 **Edit Mode**를 켭니다.
3. 작업 표시줄에서 **Build Content**를 클릭합니다.
4. **Learning Tools** 서브메뉴에서 **FastComments**를 클릭합니다. Learning Tools 서브메뉴는 관리자가 도구를 등록한 후 LTI 1.3 도구 배치로 채워집니다. 보이지 않는다면 아래의 주의사항 섹션을 참조하세요.
5. **Create FastComments** 양식에서 다음을 설정합니다:
   - **Name**: 학생이 콘텐츠 영역에서 보게 될 레이블.
   - **Description**: 포함된 스레드 위에 표시되는 선택적 텍스트.
   - **Permit Users to View this Content**: 사용 가능 여부 토글(예/아니요).
   - **Track Number of Views**: Blackboard의 항목별 조회 통계를 원하면 활성화합니다. FastComments는 자체 분석을 별도로 운영합니다.
   - **Date and Time Restrictions**: 선택적 **Display After** / **Display Until** 기간.
6. 제출합니다. 도구가 콘텐츠 영역에 클릭 가능한 항목으로 표시됩니다.

#### Embedding Inside an Item or Document

두 Course View 모두에서 강사는 콘텐츠 편집기의 LTI Advantage 버튼을 통해 항목, 문서 또는 모든 리치 텍스트 필드의 본문 안에 FastComments를 인라인으로 임베드합니다.

Ultra Course View:

1. **Document**를 만들거나 편집합니다.
2. 스레드를 표시하려는 문서 본문 안에서 **Add content**를 클릭합니다.
3. 편집기 도구 모음에서 **Insert content** 메뉴를 열고 **Content Market**(LTI Advantage / Deep Linking 진입점)을 클릭합니다.
4. **FastComments**를 선택합니다. FastComments는 딥링크 페이로드를 반환하고 Blackboard는 커서 위치의 문서 본문에 임베디드 블록을 삽입합니다.
5. 문서를 저장합니다. 학생은 스크롤하여 해당 블록을 지나갈 때 인라인으로 렌더링된 스레드를 봅니다.

Original Course View:

1. 리치 텍스트 본문이 있는 항목을 편집합니다.
2. Content Editor 도구 모음에서 **Add Content** 더하기 아이콘을 클릭하고 **Content Market**을 선택합니다(구형 Q4 2024 CU에서는 **Add Content from External Tool**로 표기).
3. **FastComments**를 선택합니다. 편집기는 딥링크 리소스를 참조하는 자리표시자 블록을 삽입합니다.
4. 항목을 제출합니다.

각 딥링크 임베드는 고유한 FastComments 스레드를 생성하므로, 하나의 항목에 두 개의 FastComments 블록이 포함되면 두 개의 독립된 댓글 스트림이 생성됩니다.

#### Visibility, Release Conditions, and Group Restrictions

FastComments 콘텐츠 항목은 그 위에 적용되는 액세스 제어 규칙에 대해 다른 Blackboard 콘텐츠 항목과 동일하게 동작합니다.

- Ultra: 행의 가시성 선택기(**Visible to students**, **Hidden from students**, **Conditional availability**)를 클릭합니다. Conditional availability는 날짜/시간 창, 성적부 항목에 대한 성과 규칙, 강좌 그룹에 대한 구성원 규칙을 지원합니다.
- Original: 항목의 컨텍스트 메뉴를 열고 **Adaptive Release** 또는 **Adaptive Release: Advanced**를 선택하여 날짜, 구성원, 성적 또는 검토 상태로 도구를 제한합니다. 항목에서 **Set Group Availability**를 사용하여 특정 강좌 그룹으로 제한합니다.

FastComments는 Blackboard의 게이트 설정을 준수합니다. Blackboard가 학생에게 항목을 숨기면 해당 학생에 대해 LTI 런치는 발생하지 않으며 그 학생은 중재자 보기에도 나타나지 않습니다.

#### Gradebook Behavior

FastComments는 LTI Advantage Assignment and Grade Services를 통해 성적을 보고하지 않습니다. FastComments 콘텐츠 항목에 대한 성적 열이 자동으로 생성되지 않습니다.

기관의 Blackboard 테넌트가 채점 메타데이터와 관계없이 모든 새 콘텐츠 항목에 대해 성적부 열을 자동 생성하도록 구성된 경우, 빈 열이 어쨌든 나타납니다. 이를 숨기려면:

- Ultra: **Gradebook**을 열고 열 헤더를 클릭한 다음 **Edit**를 선택하고 **Show to students**와 **Include in calculations**를 끕니다. 또는 기관에서 채점되지 않은 항목에 대해 열 삭제를 허용하면 **Delete**를 사용합니다.
- Original: **Grade Center**를 열고 열의 체브론을 클릭한 다음 **Hide from Users (on/off)**를 선택하고 선택적으로 **Column Organization**에서 **Hide from Instructor View**를 설정합니다.

#### What Students See

학생이 FastComments 항목을 열거나 임베디드 블록으로 스크롤하면:

1. Blackboard는 FastComments로 LTI 1.3 메시지를 런치합니다. 학생은 Blackboard의 자격 증명(이름, 이메일, 아바타, 역할)을 사용해 로그인 폼을 보지 않고 SSO로 로그인됩니다.
2. 댓글 스레드가 iframe 안에 렌더링됩니다. 스레딩, 답글, 멘션 및 반응은 FastComments에서 구성한 댓글 위젯 설정에 따라 모두 사용 가능합니다.
3. 학생의 댓글은 그들의 Blackboard 계정에 귀속됩니다. 학생이 이후 Blackboard에서 이름이나 사진을 수정하면 다음 런치 시 FastComments 프로필이 업데이트됩니다.

Blackboard에서 FastComments로의 역할 매핑:

- **System Administrator** 및 **Course Builder**는 FastComments의 **admin**으로 매핑됩니다.
- **Instructor** 및 **Teaching Assistant**는 FastComments의 **moderator**로 매핑됩니다.
- **Student**, **Guest**, 및 **Observer**는 FastComments의 **commenter**로 매핑됩니다.

중재자는 스레드의 모든 댓글에 대해 핀, 숨기기, 차단, 삭제 등의 중재 컨트롤을 인라인으로 봅니다.

#### Thread Scoping

FastComments는 각 스레드를 **(Blackboard host, course ID, resource link ID)**로 스코핑합니다. 같은 강좌에 있는 두 FastComments 항목은 두 개의 스레드를 생성합니다. 동일한 항목이 두 강좌 쉘에 복사된 경우(예: 코스 복사를 통해) 각각의 스레드는 두 개가 생성되며, 이는 복사 중 Blackboard가 새로운 resource link ID를 발급하기 때문입니다. 코스 복사 간에 스레드를 공유하려면, 복사하기 전에 FastComments에서 명시적인 스레드 URN을 구성한 Deep Linking을 사용하세요.

#### Blackboard-Specific Gotchas

**Build Content 메뉴(Original) 또는 Content Market(Ultra)에서 FastComments 타일이 없음.** 관리자가 도구를 승인했지만 관련 배치를 차단하는 기관 정책을 남겨둔 것입니다. **Administrator Panel** > **Integrations** > **LTI Tool Providers**로 이동하여 FastComments 항목을 편집하고 **Course Content Tool**(Original) 및 **Course Content Tool - allow students** / **Deep Linking content tool**(Ultra) 배치가 모두 활성화되어 있는지 확인하세요. 저장한 후 강좌 페이지를 새로고침합니다.

**런치 시 "Tool not configured for this context" 또는 "Tool is not deployed" 오류.** 동적 등록 중에 등록된 배포 범위가 강좌가 속한 기관 컨텍스트와 일치하지 않습니다. Blackboard의 도구 제공자 항목에서 **Deployment ID**가 이 테넌트에 대한 FastComments의 LTI 1.3 Configuration 페이지에 표시된 값과 일치하는지 확인하세요. 다르면 배치를 삭제하고 새로운 등록 URL에서 동적 등록을 다시 실행하세요.

**Iframe 높이가 고정되어 보이거나 내용이 잘림.** 일부 Blackboard 테넌트는 기본 LTI iframe-resize postMessage를 차단하는 엄격한 Content Security Policy를 적용합니다. FastComments는 호환성을 극대화하기 위해 Canvas 스타일의 `lti.frameResize` 메시지와 IMS 규격형 `org.imsglobal.lti.frameResize` 메시지를 모두 전송하지만, 테넌트 수준의 CSP 재정의가 부모 리스너를 차단합니다. 관리자에게 `*.fastcomments.com`이 LTI 도구 허용 목록에 포함되어 있는지 및 postMessage 이벤트를 제거하는 사용자 정의 CSP 헤더가 없는지 확인해 달라고 요청하세요. 그러면 별도의 추가 구성 없이 리사이즈가 작동합니다.

**코스 복사가 스레드를 복제함.** Blackboard 코스 복사는 LTI 배치에 대해 새로운 resource link ID를 발급하므로 복사된 코스는 빈 스레드로 시작합니다. 이는 예상되는 동작입니다. 복사된 코스가 원본 스레드를 상속하도록 하려면 복사 전에 명시적 스레드 URN으로 Deep Linking을 설정하거나 FastComments 지원에 연락해 스레드 ID를 대량으로 재매핑하도록 요청하세요.

**학생이 런치 시 일반적인 Blackboard 오류를 봄.** 원인은 누락되었거나 오래된 `email` 클레임입니다. FastComments에 대한 기관 정책에서 **User Fields to Send** 아래에 **Role**, **Name**, 및 **Email Address**가 활성화되어 있는지 확인하세요. 저장한 후 새 브라우저 세션에서 다시 런치해 보세요.