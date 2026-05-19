이 페이지에서는 관리자가 도구를 등록하고 배포(deployment)를 생성한 이후 Brightspace 강좌에 FastComments를 추가하는 방법을 다룹니다. 도구가 아직 등록되지 않았다면 먼저 D2L 등록 가이드를 참조하세요.

<div class="screenshot white-bg">
    <div class="title">Brightspace의 유닛 토픽에 임베드된 FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments running inside a Brightspace unit, showing threaded comments and an @-mention picker" />
</div>

Brightspace는 두 가지 콘텐츠 작성 환경을 제공합니다: **Classic Content**와 **New Content Experience**(**Lessons**라고도 함). 둘 다 FastComments를 노출하지만 메뉴 경로가 다릅니다. 아래 각 섹션은 차이가 있는 부분을 모두 다룹니다.

#### FastComments 도구 찾기

FastComments 도구는 강좌 콘텐츠 편집기 내에서 두 곳에 나타납니다:

1. 모듈/유닛의 **Add Existing** 버튼(구버전 Brightspace에서는 **Add Existing Activities**)에서 접근하는 활동 선택기(activity picker). 최신 Brightspace에서는 FastComments가 선택기 목록에 바로 나타나며, 구버전에서는 **External Learning Tools** 하위메뉴에 중첩되어 있습니다. 어느 경로로든 FastComments를 추가하면 독립적인 토픽으로 생성됩니다.
2. HTML 편집기 내의 **Insert Stuff** 대화상자에서 **LTI Advantage** 항목. 이는 LTI 딥링크(Deep Linking) 흐름을 통해 HTML 토픽 안에 FastComments를 인라인으로 임베드합니다.

두 선택기 중 어느 쪽에도 FastComments가 보이지 않으면 배포가 해당 강좌를 포함하는 조직 단위(org unit)에 대해 활성화되어 있지 않은 것입니다. Brightspace 관리자에게 요청하여 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**로 이동해 배포를 열고 **Org Units** 아래에 강좌의 org unit(또는 상위 org unit)을 추가해 달라고 하세요.

#### 모듈에 FastComments를 토픽으로 추가하기

Classic Content:

1. 강좌를 열고 네비게이션 바에서 **Content**를 클릭합니다.
2. 토론을 넣을 모듈을 선택합니다(또는 **Add a module**로 새 모듈을 만듭니다).
3. **Add Existing**을 클릭합니다(구버전 Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. 선택기에서 **FastComments**를 클릭합니다. Brightspace가 모듈에 토픽을 생성하고 콘텐츠 보기로 돌아갑니다.
5. 새 토픽을 클릭합니다. 인라인 제목 편집기를 사용하여 `FastComments Discussion`처럼 설명적인 이름으로 바꿉니다.

New Content Experience (Lessons):

1. 강좌를 열고 **Content**를 클릭합니다.
2. 토론을 넣을 유닛과 레슨을 엽니다.
3. **Add** > **Existing Activity**를 클릭하고 **FastComments**를 선택합니다(구버전 Brightspace: **External Learning Tools** 하위에 중첩되어 있음).
4. 활동이 레슨에 추가됩니다.
5. 활동 제목을 클릭하여 이름을 바꿉니다.

어떤 사용자(강사나 학생이든)가 토픽을 처음 열면 FastComments가 해당 리소스 링크에 대한 스레드를 초기화합니다. 스레드는 리소스 링크 ID에 바인딩되므로 토픽의 이름을 바꾸거나 이동해도 불러오는 스레드는 변경되지 않습니다.

#### HTML 토픽에 FastComments를 인라인 임베드하기

읽기 자료, 비디오 또는 기타 콘텐츠 아래에 댓글이 동일한 토픽 페이지 안에 나타나길 원할 때 이 흐름을 사용하세요(별도 토픽으로 만들지 않음).

1. 모듈/레슨에서 HTML 토픽을 열거나 새로 만듭니다.
2. Brightspace HTML 편집기를 열려면 **Edit HTML**을 클릭합니다.
3. 댓글 스레드가 나타나길 원하는 위치에 커서를 둡니다.
4. 편집기 툴바의 퍼즐 조각 아이콘인 **Insert Stuff** 버튼을 클릭합니다.
5. Insert Stuff 대화상자에서 **LTI Advantage**로 스크롤하여 **FastComments**를 클릭합니다.
6. FastComments가 딥링크 선택기를 엽니다. 배치(기본 옵션은 콘텐츠 토론에 적합함)를 확인하고 **Insert** 또는 **Continue**를 클릭합니다.
7. Brightspace가 플레이스홀더 블록을 포함한 HTML 편집기로 돌아갑니다(이는 LTI 실행을 나타냄). 토픽에서 **Save and Close**를 클릭합니다.

토픽이 로드될 때 Brightspace는 플레이스홀더를 iframe으로 교체하고 LTI를 통해 FastComments를 자동 실행합니다. 학생들은 인라인으로 토론 스레드를 보게 됩니다.

하나의 HTML 토픽은 여러 개의 딥링크된 FastComments 임베드를 포함할 수 있습니다. 각 임베드는 각기 다른 리소스 링크 ID를 생성하므로 각 임베드는 자체 스레드를 받습니다.

#### 모듈 토픽 대 인라인 퀵링크

다음 경우에는 **모듈 토픽** 방식을 선택하세요:

- 토론이 해당 모듈 단계의 주요 활동인 경우.
- 토픽이 Brightspace의 목차, 완료 추적 및 Class Progress에 나타나길 원하는 경우.

다음 경우에는 **인라인 임베드** 방식을 선택하세요:

- 댓글이 동일한 페이지의 다른 콘텐츠 아래에 있어야 하는 경우.
- 목차에 별도의 완료 추적 항목이 나타나면 안 되는 경우.

#### 가시성, 초안(Draft), 공개 조건(Release Conditions)

새 FastComments 토픽은 기본적으로 학생들에게 보입니다. 설정하는 동안 숨기려면:

1. 콘텐츠 편집기에서 토픽 제목(클래식) 또는 활동의 점 3개 메뉴(뉴 콘텐츠 경험)를 클릭합니다.
2. 상태를 **Draft**로 설정(클래식)하거나 **Visibility**를 끕니다(뉴 콘텐츠 경험).

Draft 토픽은 학생에게 보이지 않습니다. 강사와 TA는 "Draft" 배지와 함께 여전히 볼 수 있습니다.

토픽을 특정 그룹이나 섹션으로 제한하려면:

1. 토픽을 엽니다.
2. 토픽 제목 메뉴 > **Edit Properties In-place**(클래식) 또는 **Edit** > **Restrictions**(뉴 콘텐츠 경험)을 클릭합니다.
3. **Release Conditions** 아래에서 **Create**를 클릭합니다.
4. **Group enrollment** 또는 **Section enrollment**를 선택하고 그룹/섹션을 선택한 후 저장합니다.

Release 조건은 FastComments 자체의 역할 매핑과 함께 적용됩니다. 토픽을 볼 수 없는 학생은 LTI 실행도 받지 못합니다.

#### 학생들이 처음 실행할 때 보는 것

학생이 토픽을 클릭하거나 임베드된 HTML 토픽을 로드하면:

1. Brightspace가 백그라운드에서 LTI 1.3 런치를 수행합니다.
2. FastComments는 학생의 이름, 이메일, 아바타 URL, LMS 역할을 받아 자동으로 로그인시킵니다. FastComments 로그인 프롬프트는 없습니다.
3. 해당 리소스 링크의 댓글 스레드가 Brightspace iframe 내에 렌더링됩니다.

런치 시 역할 매핑:

- Brightspace의 `Administrator`는 스레드에 대한 FastComments의 관리자(admin)가 됩니다(전체 중재, 삭제, 차단 및 구성 접근 권한).
- Brightspace의 `Instructor`는 FastComments의 중재자(moderator)가 됩니다(고정, 숨김, 삭제, 차단).
- 그 외 모든 역할(`Learner`, `TeachingAssistant` 등)은 일반 댓글 작성자(commenter)가 됩니다.

댓글은 학생의 Brightspace 계정에 귀속됩니다. 학생이 Brightspace에서 이름이나 아바타를 편집하면 다음 LTI 런치에서 변경 사항이 동기화됩니다.

#### 공용 접근 잠금(권장)

기본적으로 FastComments의 댓글 데이터는 공개 읽기 가능(publicly readable)입니다. 스레드의 URL이나 API 엔드포인트를 추측할 수 있는 사람은 Brightspace 외부에서도 댓글을 볼 수 있습니다. 코스 토론의 경우 보통 등록된 학습자만 보도록 제한하는 것이 좋습니다.

귀하의 <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">widget customization page</a>를 열고 **Require SSO To View Comments**가 활성화된 규칙을 생성한 다음 보안 수준을 **Secure SSO**로 설정하여 스레드가 서명된 LTI 런치를 통해서만 로드되도록 하세요.

[Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments)에서 단일 도메인이나 페이지로 규칙 범위를 한정하는 방법을 포함한 전체 절차를 확인하세요.

#### iframe 높이 및 리사이즈

FastComments는 모든 스레드 렌더 및 콘텐츠 변경(새 댓글, 답글 확장) 시 `org.imsglobal.lti.frameResize` postMessage를 발신합니다. Brightspace는 이 메시지를 수신하여 iframe 높이를 조정하므로 스레드가 잘리거나 내부 스크롤바가 표시되지 않습니다.

iframe이 짧은 고정 높이로 남아 있는 경우:

- 코스가 HTTPS로 로드되고 있는지 확인하세요. Brightspace의 postMessage 리스너는 혼합 콘텐츠(mixed-content) 프레임을 거부합니다.
- 브라우저 확장 프로그램이 postMessage 채널을 차단하고 있지 않은지 확인하세요.
- HTML 토픽의 인라인 임베드인 경우, 주변 HTML이 iframe을 고정 높이 컨테이너로 감싸지 않아야 합니다. 상위 요소의 `style="height: ..."` 같은 인라인 스타일을 제거하세요.

#### Brightspace 관련 주의사항

**Add Existing 선택기에 도구가 표시되지 않음.** 배포가 해당 강좌의 org unit에 대해 활성화되어 있지 않습니다. 관리자가 배포의 **Org Units** 목록에 org unit(또는 상위 org unit)을 추가해야 합니다. 도구 등록만으로는 충분하지 않습니다; 배포가 어떤 강좌들이 도구를 보게 될지를 결정합니다.

**런치 시 `deployment_id` 불일치.** FastComments는 등록 시 처음 본 `deployment_id`를 TOFU(Trust On First Use) 방식으로 고정합니다. 관리자가 원래 배포를 삭제하고 새 배포를 만들면 새 배포에서의 런치는 배포 불일치 오류로 거부됩니다. 해결 방법은 FastComments를 다시 등록하는 것입니다(새 등록 URL을 생성하여 (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기에서 가져오기</a>) 동적 등록을 다시 실행하세요); 기존 구성 레코드는 교체됩니다.

**툴은 실행되지만 "Invalid LTI launch"가 표시됨.** 강좌가 배포가 적용되는 테넌트/조직 구조와 다른 곳에 있거나 배포가 등록 후 비활성화되었을 수 있습니다. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 토글과 배포의 org unit 목록을 다시 확인하세요.

**FastComments 내부에 이름과 역할이 없음.** Brightspace는 Names and Role Provisioning Services(NRPS) 클레임을 포함한 LTI 런치를 제공합니다. 만약 코스가 오래된 LTI 1.1 링크에서 업그레이드된 경우 런치에 `name` 및 `email` 클레임이 없을 수 있습니다. FastComments 토픽을 **Add Existing**로 다시 추가하세요(기존 링크를 마이그레이션하지 마세요). 그래야 런치가 LTI 1.3을 사용합니다.

**임베드가 자동 SSO 대신 로그인 화면을 표시함.** HTML 토픽이 **Insert Stuff** > **LTI Advantage**를 통해 삽입된 것이 아니라 FastComments를 가리키는 일반 `<iframe>`으로 삽입된 경우입니다. 일반 iframe은 LTI 런치를 건너뛰어 사용자를 공개 FastComments 페이지로 보냅니다. 해당 iframe을 삭제하고 Insert Stuff 흐름을 통해 다시 삽입하세요.