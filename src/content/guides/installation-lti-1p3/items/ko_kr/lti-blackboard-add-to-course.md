관리자가 FastComments를 LTI 1.3 Advantage 도구로 등록하고 기관 정책을 승인하면, 강사는 표준 Blackboard 배치 지점을 통해 강좌에 이를 추가합니다. 정확한 절차는 Ultra Course View와 Original Course View에서 서로 다르므로 두 가지 모두 아래에서 다룹니다.

#### Ultra Course View

Ultra Course View는 2026년 기준 Blackboard Learn SaaS의 기본 뷰입니다.

1. 강좌를 열고 **Course Content** 페이지로 이동합니다.
2. 개요에서 댓글 스레드가 위치할 곳에 마우스를 올리거나 탭한 후 보라색 **+**(콘텐츠 추가) 버튼을 클릭합니다.
3. **Content Market**을 선택합니다. Content Market 패널에는 기관에서 승인된 모든 LTI 도구와 Building Block 배치가 나열됩니다.
4. **FastComments** 타일을 찾아 클릭합니다. Blackboard가 **+** 메뉴를 연 위치에 콘텐츠 항목을 생성합니다.
5. 항목은 기본적으로 "Visible to students" 항목으로 개요에 배치됩니다. 이는 **Hide from students**가 개인 기본 설정으로 꺼져 있는 강사의 경우입니다. 기본 설정이 **Hidden**이면 항목이 숨겨진 상태로 생성되며, 준비가 되면 항목 행에서 표시 여부 선택기를 전환하면 됩니다.
6. 항목 이름을 바꾸려면 개요에서 제목을 클릭하고 새 레이블을 입력합니다. 개요에서 학생에게 보이는 제목은 FastComments 스레드 식별자와 독립적이므로 언제든지 이름을 바꿔도 안전합니다.

**Content Market** 옵션이 보이지 않는다면, 기관에서 해당 배치를 숨겨둔 것입니다. 동일한 **+** 메뉴의 **More tools** 아래 **LTI Tools** 그룹을 통해서도 동일한 선택기에 접근할 수 있습니다.

#### Original Course View

Original Course View는 Learn SaaS에서 여전히 지원되며, Q4 2024 CU 릴리스 라인의 자체 호스팅 Learn 9.1 사이트에서는 기본 환경입니다.

1. 강좌를 열고 **Content Area**(예: 강좌 메뉴의 기본 **Information** 또는 **Content** 영역)에 들어갑니다.
2. 페이지 오른쪽 상단의 토글로 **Edit Mode**를 켭니다.
3. 작업 표시줄에서 **Build Content**를 클릭합니다.
4. **Learning Tools** 하위 메뉴에서 **FastComments**를 클릭합니다. Learning Tools 하위 메뉴는 관리자가 도구를 등록한 후 LTI 1.3 도구 배치에서 채워집니다. 보이지 않으면 아래의 주의 사항 섹션을 참조하세요.
5. **Create FastComments** 양식에서 다음을 설정합니다:
   - **Name**: 콘텐츠 영역에서 학생에게 보이는 레이블입니다.
   - **Description**: 임베디드 스레드 위에 표시되는 선택적 텍스트입니다.
   - **Permit Users to View this Content**: 예/아니요 가용성 토글입니다.
   - **Track Number of Views**: Blackboard의 항목별 조회 통계를 원하는 경우 활성화합니다. FastComments는 자체 분석을 독립적으로 실행합니다.
   - **Date and Time Restrictions**: 선택적 **Display After** / **Display Until** 기간입니다.
6. 제출합니다. 도구가 콘텐츠 영역에 클릭 가능한 항목으로 나타납니다.

#### 항목 또는 문서 내 임베딩

두 강좌 뷰 모두에서 강사는 Content Editor의 LTI Advantage 버튼을 통해 Item, Document 또는 모든 리치 텍스트 필드 본문 안에 FastComments를 인라인으로 임베드할 수 있습니다.

Ultra Course View:

1. **Document**를 생성하거나 편집합니다.
2. 스레드가 나타나길 원하는 문서 본문 안에서 **Add content**를 클릭합니다.
3. 편집기 툴바에서 **Insert content** 메뉴를 열고 **Content Market**(LTI Advantage / Deep Linking 진입점)을 클릭합니다.
4. **FastComments**를 선택합니다. FastComments가 딥 링크 페이로드를 반환하고 Blackboard가 커서 위치의 문서 본문에 임베디드 블록을 삽입합니다.
5. 문서를 저장합니다. 학생은 스크롤하며 지나갈 때 인라인으로 렌더링된 스레드를 봅니다.

Original Course View:

1. 리치 텍스트 본문이 있는 항목을 편집합니다.
2. Content Editor 툴바에서 **Add Content** 플러스 아이콘을 클릭하고 **Content Market**(구버전 Q4 2024 CU에서는 **Add Content from External Tool**으로 표시됨)을 선택합니다.
3. **FastComments**를 선택합니다. 편집기가 딥 링크된 리소스를 참조하는 자리표시자 블록을 삽입합니다.
4. 항목을 제출합니다.

각 딥 링크 임베드는 자체 FastComments 스레드를 생성하므로, 두 개의 FastComments 블록이 임베드된 Item은 두 개의 독립적인 댓글 스트림을 가집니다.

#### 가시성, 공개 조건, 그룹 제한

FastComments 콘텐츠 항목은 그 위에 적용되는 접근 제어 규칙 측면에서 다른 Blackboard 콘텐츠 항목과 동일하게 동작합니다.

- Ultra: 행의 표시 여부 선택기를 클릭합니다(**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability는 날짜/시간 기간, 성적표 항목에 대한 성과 규칙, 강좌 그룹에 대한 멤버 규칙을 지원합니다.
- Original: 항목의 컨텍스트 메뉴를 열고 **Adaptive Release** 또는 **Adaptive Release: Advanced**를 선택하여 날짜, 멤버십, 성적 또는 검토 상태로 도구 접근을 제한합니다. 항목에서 **Set Group Availability**를 사용하여 특정 강좌 그룹으로 제한할 수 있습니다.

FastComments는 Blackboard의 게이트가 결정한 내용을 존중합니다. Blackboard가 학생에게 항목을 숨기면, 해당 학생에 대해서는 LTI 실행이 발생하지 않으며 중재자 뷰에도 나타나지 않습니다.

#### 성적표 동작

FastComments는 LTI Advantage Assignment and Grade Services를 통해 성적을 보고하지 않습니다. FastComments 콘텐츠 항목에 대해 성적 열이 자동 생성되지 않습니다.

Blackboard 테넌트가 채점 메타데이터와 무관하게 모든 새 콘텐츠 항목에 대해 성적표 열을 자동 생성하도록 구성되어 있다면, 빈 열이 그래도 나타납니다. 이를 숨기려면:

- Ultra: **Gradebook**을 열고 열 헤더를 클릭한 후 **Edit**를 선택하고 **Show to students**와 **Include in calculations**를 끕니다. 또는 기관이 채점되지 않은 항목의 열 삭제를 허용하는 경우 **Delete**를 사용합니다.
- Original: **Grade Center**를 열고 열의 셰브론을 클릭한 후 **Hide from Users (on/off)**를 선택하고, 필요시 **Column Organization** 아래에서 **Hide from Instructor View**를 선택합니다.

#### 학생에게 보이는 내용

학생이 FastComments 항목을 열거나 임베디드 블록으로 스크롤하면:

1. Blackboard가 FastComments로 LTI 1.3 메시지를 실행합니다. 학생은 로그인 양식을 보지 않고 Blackboard 신원(이름, 이메일, 아바타, 역할)을 통해 SSO로 로그인됩니다.
2. 댓글 스레드가 iframe에 렌더링됩니다. 스레딩, 답글, 멘션, 반응은 모두 FastComments에서 구성된 댓글 위젯 설정에 따라 사용할 수 있습니다.
3. 댓글은 학생의 Blackboard 계정으로 귀속됩니다. 학생이 나중에 Blackboard에서 이름이나 사진을 수정하면, 다음 실행 시 FastComments 프로필이 업데이트됩니다.

Blackboard에서 FastComments로의 역할 매핑:

- **System Administrator**와 **Course Builder**는 FastComments **admin**에 매핑됩니다.
- **Instructor**와 **Teaching Assistant**는 FastComments **moderator**에 매핑됩니다.
- **Student**, **Guest**, **Observer**는 FastComments **commenter**에 매핑됩니다.

중재자는 스레드의 모든 댓글에 인라인으로 중재 컨트롤(고정, 숨기기, 차단, 삭제)을 볼 수 있습니다.

#### 공개 접근 잠그기 (권장)

기본적으로 FastComments 댓글 데이터는 공개적으로 읽을 수 있습니다. 스레드의 URL이나 API 엔드포인트를 추측할 수 있는 사람은 Blackboard 외부에서도 댓글을 볼 수 있습니다. 강좌 토론에서는 거의 확실히 조회를 수강생으로만 제한하고 싶을 것입니다.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">위젯 사용자 지정 페이지</a>를 열고 **Require SSO To View Comments**가 활성화된 규칙을 생성한 다음, 보안 수준을 **Secure SSO**로 설정하여 스레드가 서명된 LTI 실행을 통해서만 로드될 수 있도록 합니다.

규칙을 단일 도메인이나 페이지로 범위 지정하는 방법을 포함한 전체 과정은 [Single-Sign-On으로 댓글 스레드 보호하기](/guide-customizations-and-configuration.html#sso-require-to-view-comments)를 참조하세요.

#### 스레드 범위 지정

FastComments는 각 스레드를 **(Blackboard 호스트, 강좌 ID, 리소스 링크 ID)**로 범위 지정합니다. 동일한 강좌의 두 FastComments 항목은 두 개의 스레드를 생성합니다. 동일한 항목이 두 강좌 셸에 복사되면(예: 강좌 복사를 통해) 두 개의 스레드가 생성됩니다. Blackboard가 복사 중에 새로운 리소스 링크 ID를 발급하기 때문입니다. 강좌 복사 간에 공유 스레드를 유지하려면, 복사를 실행하기 전에 FastComments에서 명시적 스레드 URN이 구성된 Deep Linking을 사용하세요.

#### Blackboard 관련 주의 사항

**Build Content 메뉴(Original) 또는 Content Market(Ultra)에서 FastComments 타일이 없음.** 관리자가 도구를 승인했지만 관련 배치를 차단하는 기관 정책이 남아 있는 경우입니다. **Administrator Panel** > **Integrations** > **LTI Tool Providers**로 이동하여 FastComments 항목을 편집하고 **Course Content Tool**(Original)과 **Course Content Tool - allow students** / **Deep Linking content tool**(Ultra) 배치가 모두 활성화되어 있는지 확인합니다. 저장하고 강좌 페이지를 새로 고칩니다.

**실행 시 "Tool not configured for this context" 또는 "Tool is not deployed" 오류.** 동적 등록 중에 등록된 배포 범위가 강좌가 속한 기관 컨텍스트와 일치하지 않습니다. Blackboard의 도구 제공자 항목에서 **Deployment ID**가 FastComments의 LTI 1.3 Configuration 페이지에서 이 테넌트에 대해 표시하는 것과 일치하는지 확인합니다. 다르다면 배치를 삭제하고 새 등록 URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기서 받으세요</a>)에서 동적 등록을 다시 실행합니다.

**iframe 높이가 고정된 것처럼 보이거나 콘텐츠가 잘림.** 일부 Blackboard 테넌트는 기본 LTI iframe 크기 조정 postMessage를 차단하는 엄격한 Content Security Policy로 제공됩니다. FastComments는 호환성을 극대화하기 위해 Canvas 스타일의 `lti.frameResize` 메시지와 IMS 사양 형식의 `org.imsglobal.lti.frameResize` 메시지를 모두 내보내지만, 테넌트 수준의 CSP 재정의가 부모 리스너를 차단합니다. 관리자에게 `*.fastcomments.com`이 LTI 도구 허용 목록에 있는지, 그리고 사용자 지정 CSP 헤더가 postMessage 이벤트를 제거하지 않는지 확인해 달라고 요청하세요. 그러면 추가 구성 없이 크기 조정이 작동합니다.

**강좌 복사가 스레드를 복제함.** Blackboard 강좌 복사는 LTI 배치에 새 리소스 링크 ID를 발급하므로, 복사된 강좌는 빈 스레드로 시작합니다. 이는 예상된 동작입니다. 복사된 강좌가 원본 스레드를 상속받아야 한다면, 복사 전에 명시적 스레드 URN으로 Deep Linking을 설정하거나, FastComments 지원팀에 문의하여 스레드 ID를 대량으로 다시 매핑하세요.

**학생이 실행 시 일반적인 Blackboard 오류를 봄.** 원인은 누락되었거나 오래된 `email` 클레임입니다. FastComments에 대한 기관 정책에서 **User Fields to Send** 아래 **Role**, **Name**, **Email Address**가 활성화되어 있는지 확인합니다. 저장한 후 새 브라우저 세션에서 다시 실행합니다.