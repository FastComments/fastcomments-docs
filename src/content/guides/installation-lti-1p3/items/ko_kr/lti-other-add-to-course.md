FastComments가 플랫폼에 등록되면 강사는 플랫폼의 표준 외부 도구 흐름을 사용하여 강좌 콘텐츠에 추가합니다. 이 페이지는 Sakai 23.x 및 Schoology Enterprise를 다룹니다.

#### 공개 액세스 차단 (권장)

기본적으로 FastComments의 댓글 데이터는 두 플랫폼 모두에서 공개적으로 읽을 수 있습니다. 스레드의 URL이나 API 엔드포인트를 추측할 수 있는 사람은 Sakai 또는 Schoology 외부에서도 해당 스레드의 댓글을 볼 수 있습니다. 강좌 토론의 경우 수강 등록된 학생만 보도록 보기 권한을 제한하는 것이 거의 확실히 필요합니다.

<a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">위젯 사용자화 페이지</a>를 열고 **Require SSO To View Comments**가 활성화된 규칙을 생성한 다음 보안 수준을 **Secure SSO**로 설정하여 서명된 LTI 런치를 통해서만 스레드를 로드할 수 있게 하세요.

전체 워크스루(특정 도메인 또는 페이지로 규칙 범위를 제한하는 방법 포함)는 [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)를 참조하세요.

#### Sakai

**1. 사이트에 FastComments 추가**

사이트 유지 관리자가 사이트별로 도구를 활성화합니다:

1. 사이트를 열고 왼쪽 네비게이션에서 **Site Info**를 클릭합니다.
2. **Manage Tools**를 클릭합니다.
3. **External Tools** 목록으로 스크롤하여 **FastComments**를 토글하여 켭니다.
4. **Continue**를 클릭하고 도구 목록을 검토한 다음 **Finish**를 클릭합니다.

이제 FastComments가 사이트의 왼쪽 네비게이션 항목으로 표시됩니다.

**2. 왼쪽 네비게이션 항목 순서 변경**

**Site Info** > **Tool Order**로 이동합니다. **FastComments**를 원하는 위치로 드래그한 다음 **Save**를 클릭합니다. 이 화면에서 네비게이션 레이블을 변경하거나 학생에게 보이지 않도록 숨길 수도 있습니다.

**3. Lessons 페이지에 인라인으로 임베드**

FastComments를 별도의 왼쪽 네비게이션 도구로 두지 않고 Lessons 페이지 내에 직접 배치하려면:

1. 사이트에서 **Lessons** 도구를 엽니다.
2. **Add Content** > **Add External Tool**을 클릭합니다.
3. 목록에서 **FastComments**를 선택합니다.
4. 등록 중 FastComments가 Deep Linking을 광고했다면 Sakai는 도구의 콘텐츠 선택기를 열어 스레드를 선택하거나 라벨을 지정할 수 있게 합니다. Deep Linking이 광고되지 않았다면 Sakai는 기본 런치 링크를 삽입합니다.
5. Lessons 항목을 저장합니다.

각 임베드 인스턴스는 해당 리소스 링크에 범위가 지정된 자체 스레드를 가집니다.

**4. 학생 액세스를 위한 권한 조정**

Sakai는 Realms를 통해 외부 도구 런치를 제어합니다. 학생이 FastComments를 런치할 수 있는지 확인하려면:

1. Sakai 관리자 계정으로 로그인하고 **Administration Workspace** > **Realms**를 엽니다.
2. 해당 realm을 엽니다(예: `!site.template.course` 또는 특정 사이트 realm).
3. `access` 역할에 `lti.launch`가 활성화되어 있고 **external.tools** 그룹의 역할 권한이 부여되어 있는지 확인합니다.
4. realm을 저장합니다.

사이트 수준 재정의의 경우, 유지 관리자는 **Site Info** > **Tool Order**에서 역할별로 FastComments의 가시성을 숨기거나 표시하여 조정할 수 있습니다.

**5. 학생이 보는 것**

학생은 FastComments 왼쪽 네비 항목을 클릭하거나(또는 임베드된 Lessons 블록으로 스크롤하여) 스레드형 댓글 뷰로 바로 이동합니다. SSO는 자동으로 처리됩니다: Sakai는 LTI 런치에서 사용자의 신원을 전송하고 FastComments는 해당 Sakai 계정으로 사용자를 로그인시킵니다.

역할 매핑:

- Sakai `Instructor` -> FastComments 모더레이터
- Sakai `Admin` (Administration Workspace의 admin) -> FastComments 관리자
- Sakai `Student` / `access` -> FastComments 댓글 작성자

**6. Sakai 유의사항**

- **Manage Tools에 도구가 보이지 않음.** FastComments가 External Tools 목록에 표시되지 않으면 Sakai 관리자가 도구 레지스트리(**Administration Workspace** > **External Tools** > **FastComments**)를 열고 **Stealthed**를 `false`로 설정해야 합니다. Stealthed된 도구는 사이트별 Manage Tools 선택기에서 숨겨집니다.
- **공유 세션 브라우저에서 런치가 실패함.** Sakai의 포털 CSRF 토큰은 브라우저 세션에 바인딩됩니다. 학생이 다른 탭에서 두 개의 Sakai 사이트에 로그인되어 있거나 세션이 오래된 경우 런치가 403을 반환할 수 있습니다. 해결 방법: 다른 Sakai 탭을 닫고 로그아웃한 다음 다시 로그인하고 런치하세요. 클러스터 전반에서 이런 문제가 발생하면 관리자가 `sakai.csrf.token.cache.ttl`을 높일 수 있습니다.
- **프레임 임베딩.** 댓글 스레드가 Lessons 페이지 내에서 잘리지 않도록 `sakai.properties`에서 `lti.frameheight`가 충분히 큰지(600 이상 권장) 확인하세요.

#### Schoology

Schoology Enterprise는 두 가지 설치 시나리오가 있습니다. 도구를 강좌에 추가하기 전에 어느 시나리오에 해당하는지 확인하세요.

**1. 두 가지 설치 시나리오**

- **(a) 엔터프라이즈 수준 설치.** Schoology 시스템 관리자가 조직 수준에서 FastComments를 설치하고 모든 강좌 또는 특정 강좌 템플릿에 할당했습니다. 강사는 설치를 건너뛰고 "Add Materials"로 바로 이동합니다.
- **(b) 강사 자체 설치.** 강사가 **Course Options** > **External Tools** > **Install LTI Apps**에서 도구를 단일 강좌에 설치합니다. 자체 설치는 시스템 관리자가 먼저 조직 수준에서 FastComments 앱을 승인했어야 합니다.

**2. FastComments를 강좌 자료로 추가**

강좌 내에서:

1. 강좌를 열고 **Materials**로 이동합니다.
2. **Add Materials** > **Add File/Link/External Tool**을 클릭합니다.
3. **External Tool**을 선택합니다.
4. 등록된 도구 목록에서 **FastComments**를 선택합니다.
5. **Name**을 설정합니다(학생이 자료 목록에서 보게 되는 이름) 및 선택적 **Description**을 입력합니다.
6. **Enable Grading**(성적 전달)은 **OFF**로 유지합니다. FastComments는 Schoology로 성적을 보고하지 않으므로 성적 전달을 켜면 빈 성적표 열이 생성됩니다.
7. **Submit**을 클릭합니다.

이제 자료가 강좌 자료 목록에 표시되며 클릭하면 FastComments 스레드가 열립니다.

**3. 리치 텍스트 편집기를 통한 인라인 임베드**

시스템 관리자가 등록 중 FastComments에 대해 Deep Linking 배치를 활성화한 경우, 강사는 리치 텍스트 필드(과제 지시문, 페이지 본문, 토론 프롬프트 등) 안에 댓글 스레드를 임베드할 수 있습니다:

1. 대상 페이지에서 리치 텍스트 편집기를 엽니다.
2. 툴바에서 **External Tool**(퍼즐 조각) 아이콘을 클릭합니다.
3. **FastComments**를 선택합니다.
4. 딥링크 대화상자에서 임베드를 구성하고 **Insert**를 클릭합니다.
5. 페이지를 저장합니다.

리치 텍스트 편집기에 External Tool 버튼이 나타나지 않으면 해당 테넌트에서 이 도구의 Deep Linking이 비활성화된 것입니다. 아래의 유의사항을 참조하세요.

**4. 가시성 및 섹션 할당**

Schoology는 Course Options를 통해 섹션별로 도구 가용성을 범위 지정합니다:

1. 강좌에서 **Course Options** > **External Tools**를 클릭합니다.
2. 설치된 각 LTI 앱에 대해 강좌의 모든 섹션에 사용 가능할지 또는 특정 섹션에만 사용할지 제어합니다.
3. FastComments를 특정 섹션으로 제한하려면 도구를 볼 수 없어야 하는 섹션의 체크를 해제합니다.
4. 섹션 수준 액세스는 또한 어떤 섹션이 **Add Materials** > **External Tool** 항목을 볼 수 있는지를 제어합니다.

**5. 학생이 보는 것**

학생은 FastComments 자료를 클릭하거나(또는 인라인 임베드로 스크롤하여) 스레드형 토론으로 이동합니다. SSO는 Schoology LTI 런치를 통해 자동으로 처리되어 학생의 Schoology 계정으로 로그인됩니다.

역할 매핑:

- Schoology `Administrator` -> FastComments 관리자
- Schoology `Instructor` -> FastComments 모더레이터
- Schoology `Student` -> FastComments 댓글 작성자

**6. Schoology 유의사항**

- **엔터프라이즈 전용.** 개인 및 무료 Schoology 계정에서는 LTI 1.3 도구를 설치할 수 없습니다. 테넌트가 무료 등급인 경우 Course Options에 **External Tools** 옵션이 없습니다. FastComments를 사용하려면 Schoology Enterprise로 업그레이드하세요.
- **테넌트 기본값으로 Deep Linking 비활성화.** 일부 Schoology 테넌트는 조직 수준에서 Deep Linking 배치를 제한합니다. 이 경우 강사는 **Add Materials** > **External Tool** 흐름만 보고 리치 텍스트 편집기의 External Tool 버튼은 보지 못합니다. 인라인 임베딩을 가능하게 하려면 시스템 관리자가 **System Settings** > **Integration** > **LTI 1.3** > **FastComments**로 가서 **Content Item / Deep Linking** 배치를 활성화한 다음 저장해야 합니다.
- **섹션별 할당 재정의.** FastComments가 엔터프라이즈 수준에서 할당되었지만 강사가 **Add Materials**에서 볼 수 없다면 해당 강좌의 섹션이 조직 수준의 할당에서 제외된 것입니다. 시스템 관리자에게 섹션을 FastComments 앱 할당에 추가해 달라고 요청하세요.
- **자료 이름 vs 스레드 정체성.** Schoology에서 자료 이름을 바꿔도 댓글 스레드는 이동하지 않습니다. 스레드는 LTI 리소스 링크 ID로 키가 지정되므로 이름을 바꿔도 동일한 스레드가 유지됩니다; 자료를 삭제하고 다시 만들면 새롭고 빈 스레드가 생성됩니다.