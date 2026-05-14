이 페이지에서는 관리자가 도구를 등록하고 배포를 생성한 이후 Brightspace 코스에 FastComments를 추가하는 방법을 설명합니다. 도구가 아직 등록되지 않았다면 먼저 D2L 등록 가이드를 참조하세요.

<div class="screenshot white-bg">
    <div class="title">Brightspace의 유닛 토픽으로 임베드된 FastComments</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="Brightspace 유닛 내에서 실행되는 FastComments, 스레드형 댓글과 @-언급 선택기가 표시됨" />
</div>

Brightspace는 두 가지 콘텐츠 작성 환경을 제공합니다: **Classic Content**와 **New Content Experience**(또는 **Lessons**라고도 함). 둘 다 FastComments를 노출하지만 메뉴 경로가 다릅니다. 아래 각 섹션에서는 갈라지는 부분을 모두 다룹니다.

#### FastComments 도구 찾기

FastComments 도구는 코스 콘텐츠 편집기 내부에서 두 곳에 나타납니다:

1. 모듈/유닛의 **Add Existing** 버튼(구버전 Brightspace에서는 **Add Existing Activities**)에서 접근하는 activity picker. 현재 Brightspace 빌드에서는 FastComments가 선택기에서 직접 표시되며, 구버전에서는 **External Learning Tools** 하위 메뉴에 중첩되어 있습니다. 어느 경로든 FastComments를 단독 토픽으로 추가합니다.
2. HTML 편집기 내부의 **Insert Stuff** 대화상자에서 **LTI Advantage** 아래. 이는 LTI 딥링크 흐름을 통해 HTML 토픽 내에 FastComments를 인라인으로 임베드합니다.

두 선택기 중 어느 곳에도 FastComments가 표시되지 않는다면, 해당 코스를 포함하는 조직 단위(org unit)에 배포가 활성화되어 있지 않은 것입니다. Brightspace 관리자에게 **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments 도구 > **View Deployments**로 이동해 배포를 열고 **Org Units** 아래에 코스의 조직 단위(또는 상위 조직 단위)를 추가해 달라고 요청하세요.

#### 모듈에 FastComments를 토픽으로 추가하기

Classic Content:

1. 코스를 열고 네비게이션바에서 **Content**를 클릭합니다.
2. 토론을 넣을 모듈을 선택합니다(또는 **Add a module**로 새 모듈을 만듭니다).
3. **Add Existing**을 클릭합니다(구버전 Brightspace: **Add Existing Activities** > **External Learning Tools**).
4. 선택기에서 **FastComments**를 클릭합니다. Brightspace가 모듈에 토픽을 생성하고 콘텐츠 뷰로 돌아갑니다.
5. 새 토픽을 클릭합니다. 인라인 제목 편집기를 사용해 `FastComments Discussion`와 같이 설명적인 이름으로 바꿉니다.

New Content Experience (Lessons):

1. 코스를 열고 **Content**를 클릭합니다.
2. 토론을 넣을 유닛과 레슨을 엽니다.
3. **Add** > **Existing Activity**를 클릭하고 **FastComments**를 선택합니다(구버전 Brightspace: **External Learning Tools** 아래에 중첩).
4. 액티비티가 레슨에 추가됩니다.
5. 액티비티 제목을 클릭하여 이름을 바꿉니다.

사용자(강사 또는 학생)가 토픽을 처음 열면 FastComments가 해당 리소스 링크에 대한 스레드를 초기화합니다. 스레드는 리소스 링크 ID에 바인딩되므로 토픽 이름을 변경하거나 이동해도 로드되는 스레드는 변하지 않습니다.

#### HTML 토픽에 FastComments 인라인 임베드하기

읽기 자료, 비디오 또는 다른 콘텐츠 아래에 댓글을 같은 토픽 페이지 내에서 표시하려면 이 흐름을 사용하세요(별도의 토픽으로 만들지 않음).

1. 모듈/레슨에서 HTML 토픽을 열거나 새로 만듭니다.
2. **Edit HTML**을 클릭해 Brightspace HTML 편집기를 엽니다.
3. 댓글 스레드를 표시할 위치에 커서를 놓습니다.
4. 편집기 도구 모음에서 퍼즐 조각 아이콘인 **Insert Stuff** 버튼을 클릭합니다.
5. Insert Stuff 대화상자에서 **LTI Advantage**로 스크롤하고 **FastComments**를 클릭합니다.
6. FastComments가 딥링크 선택기를 엽니다. 배치(기본 옵션은 콘텐츠 토론에 적합)를 확인하고 **Insert** 또는 **Continue**를 클릭합니다.
7. Brightspace가 LTI 실행을 나타내는 플레이스홀더 블록과 함께 HTML 편집기로 돌아갑니다. 토픽에서 **Save and Close**를 클릭합니다.

토픽이 로드되면 Brightspace는 플레이스홀더를 iframe으로 교체하고 LTI를 통해 FastComments를 자동 실행합니다. 학생들은 인라인으로 토론 스레드를 보게 됩니다.

단일 HTML 토픽에는 여러 개의 딥링크된 FastComments 임베드가 들어갈 수 있습니다. 각 임베드는 고유한 리소스 링크 ID를 생성하므로 각각 별도의 스레드를 갖습니다.

#### 모듈 토픽 대 인라인 퀵링크

다음과 같은 경우에는 **모듈 토픽** 방식을 선택하세요:

- 해당 모듈 단계에 토론이 주요 활동인 경우.
- 토픽을 Brightspace의 목차, 완료 추적 및 Class Progress에 표시하고 싶은 경우.

다음과 같은 경우에는 **인라인 임베드** 방식을 선택하세요:

- 댓글이 동일 페이지의 다른 콘텐츠 아래에 있어야 하는 경우.
- 목차에 별도의 완료 추적 항목으로 표시되기를 원하지 않는 경우.

#### 가시성, 임시 저장(Draft), 릴리스 조건

새 FastComments 토픽은 기본적으로 학생에게 보입니다. 설정하는 동안 숨기려면:

1. 콘텐츠 편집기에서 토픽 제목(클래식) 또는 액티비티의 점 3개 메뉴(새 콘텐츠 경험)를 클릭합니다.
2. 상태를 **Draft**로 설정(클래식)하거나 **Visibility**를 끕니다(새 콘텐츠 경험).

Draft 상태의 토픽은 학생에게 보이지 않습니다. 강사와 TA는 "Draft" 배지와 함께 계속 볼 수 있습니다.

토픽을 특정 그룹이나 섹션으로 제한하려면:

1. 토픽을 엽니다.
2. 토픽 제목 메뉴 > **Edit Properties In-place**(클래식) 또는 **Edit** > **Restrictions**(새 콘텐츠 경험)를 클릭합니다.
3. **Release Conditions** 아래에서 **Create**를 클릭합니다.
4. **Group enrollment** 또는 **Section enrollment**를 선택하고 그룹/섹션을 선택한 다음 저장합니다.

릴리스 조건은 FastComments의 자체 역할 매핑과 결합됩니다. 학생이 토픽을 볼 수 없으면 LTI 실행도 받지 않습니다.

#### 학생이 처음 실행할 때 보이는 것

학생이 토픽을 클릭하거나 임베드가 포함된 HTML 토픽을 로드할 때:

1. Brightspace가 백그라운드에서 LTI 1.3 런치를 수행합니다.
2. FastComments는 학생의 이름, 이메일, 아바타 URL 및 LMS 역할을 받고 사용자를 자동으로 로그인 처리합니다. FastComments 로그인 프롬프트는 표시되지 않습니다.
3. 해당 리소스 링크의 댓글 스레드가 Brightspace iframe 내에서 렌더링됩니다.

런치 시 역할 매핑:

- Brightspace `Administrator`는 스레드에 대해 FastComments의 **admin**이 됩니다(전체 중재, 삭제, 차단 및 설정 접근).
- Brightspace `Instructor`는 FastComments의 **moderator**가 됩니다(고정, 숨기기, 삭제, 차단).
- 기타 모든 역할(`Learner`, `TeachingAssistant` 등)은 일반 댓글 작성자로 처리됩니다.

댓글은 학생의 Brightspace 계정에 귀속됩니다. 학생이 Brightspace에서 이름이나 아바타를 편집하면 다음 LTI 런치에서 변경사항이 동기화됩니다.

#### iframe 높이와 리사이즈

FastComments는 각 스레드 렌더링 시와 콘텐츠 변경 시(새 댓글, 답글 확장 등) `org.imsglobal.lti.frameResize` postMessage를 전송합니다. Brightspace는 이 메시지를 수신하여 iframe 높이를 조정하므로 스레드가 잘리거나 내부 스크롤바가 표시되지 않습니다.

iframe 높이가 짧게 고정된 상태로 유지된다면:

- 코스가 HTTPS로 로드되는지 확인하세요. Brightspace의 postMessage 수신기는 혼합 콘텐츠 프레임을 거부합니다.
- 브라우저 확장 프로그램이 postMessage 채널을 차단하고 있지 않은지 확인하세요.
- HTML 토픽에 인라인 임베드된 경우, iframe을 고정 높이 컨테이너로 감싸면 안 됩니다. 부모 요소에서 inline `style="height: ..."`를 제거하세요.

#### Brightspace 관련 주의사항

**Add Existing 선택기에 도구가 보이지 않는 경우.** 배포가 이 코스의 조직 단위에 대해 활성화되어 있지 않습니다. 관리자가 배포의 **Org Units** 목록에 조직 단위(또는 상위)를 추가해야 합니다. 도구 등록만으로는 충분하지 않습니다; 배포가 어떤 코스가 도구를 볼지 범위를 지정합니다.

**런치 시 `deployment_id` 불일치.** FastComments는 등록 시 처음 본 `deployment_id`를 TOFU 방식으로 고정합니다. 관리자가 원래 배포를 삭제하고 새 배포를 만들면 새 배포에서의 런치는 배포 불일치 오류로 거부됩니다. 해결 방법은 FastComments를 다시 등록하는 것입니다(새 등록 URL을 생성하고 동적 등록을 다시 실행); 기존 구성 레코드가 교체됩니다.

**도구는 실행되지만 "Invalid LTI launch"를 표시하는 경우.** 코스가 배포가 커버하는 테넌트/조직 구조와 다르거나, 등록 후 배포가 비활성화되었을 수 있습니다. **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > **Enabled** 토글 및 배포의 조직 단위 목록을 다시 확인하세요.

**FastComments 내에서 이름과 역할이 누락되는 경우.** Brightspace는 이름 및 역할 프로비저닝 서비스(NRPS) 클레임을 포함한 LTI 런치를 전송합니다. 코스가 오래된 LTI 1.1 링크에서 업그레이드된 경우 런치에 `name` 및 `email` 클레임이 누락될 수 있습니다. FastComments 토픽을 **Add Existing**로 다시 추가하세요(기존 링크를 마이그레이션하지 마세요). 그러면 런치가 LTI 1.3을 사용합니다.

**임베드가 자동 SSO 대신 로그인 화면을 표시하는 경우.** HTML 토픽이 **Insert Stuff** > **LTI Advantage**를 통해 삽입된 것이 아니라 FastComments를 가리키는 일반 `<iframe>`으로 삽입된 것입니다. 일반 iframe은 LTI 런치를 건너뛰어 사용자를 공용 FastComments 페이지로 안내합니다. iframe을 삭제하고 Insert Stuff 흐름을 통해 다시 삽입하세요.