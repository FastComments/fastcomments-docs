**Moodle를 사용하고 계신가요?** FastComments는 LTI 1.3보다 더 긴밀하게 통합되는 등급 동기화 훅, 더 깊은 활동 보고, 네이티브 Moodle 설정 UI를 제공하는 전용 Moodle 플러그인도 배포합니다. <a href="/guide-installation-moodle.html" target="_blank">Moodle 플러그인 설치 가이드</a>를 참조하세요. 아래의 LTI 1.3 흐름은 다른 LMS도 포함하는 단일 등록을 원하거나 Moodle 관리자가 타사 플러그인을 설치하지 않을 경우 적합한 선택입니다.

Moodle 4.0+는 External Tool 플러그인을 통해 LTI 1.3 동적 등록을 지원합니다.

#### 도구 관리 화면 열기

1. 사이트 관리자 계정으로 Moodle에 로그인합니다.
2. **사이트 관리** > **플러그인** > **활동 모듈** > **외부 도구** > **도구 관리**로 이동합니다.

#### URL 붙여넣기

**Tool URL**이라는 카드가 표시됩니다. 텍스트 필드에 FastComments 등록 URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기에서 가져오기</a>)을 붙여넣고 **Add LTI Advantage**를 클릭합니다.

Moodle은 도구의 식별 정보와 요청된 권한을 보여주는 등록 화면을 엽니다. 검토한 후 **활성화**(또는 Moodle 버전에 따라 **등록**)를 클릭하세요.

등록이 완료되면 팝업이 닫히며 새 FastComments 도구가 **도구** 목록에 **활성** 상태로 나타납니다.

#### 사용 가능하게 만들기

기본적으로 Moodle은 새 도구를 "강좌 도구" 목록에 추가하지만 활동 선택기에는 표시하지 않습니다. FastComments를 강좌 전체에 노출하려면:

1. FastComments 타일에서 톱니바퀴 아이콘을 클릭합니다.
2. **도구 구성 사용**에서 **활동 선택기 및 사전 구성된 도구로 표시**를 선택합니다.
3. 저장합니다.

이제 강사는 **활동 또는 자료 추가** > **FastComments**를 통해 어떤 강좌든 FastComments를 추가할 수 있습니다.