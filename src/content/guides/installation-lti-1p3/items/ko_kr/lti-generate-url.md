#### LTI 1.3 구성으로 이동

FastComments에 로그인한 다음 <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">LTI 1.3 구성 페이지</a>로 이동합니다.

계정에 아직 LTI 접근 권한이 없으면 "이 계정에 대해 LTI가 활성화되어 있지 않습니다" 메시지가 표시됩니다 - 해당 플랜에 LTI를 활성화하려면 지원팀에 문의하세요.

#### 플랫폼 선택 (선택 사항)

**동적 등록 URL 생성** 아래에서 **플랫폼** 드롭다운을 사용하여 연결할 LMS를 FastComments에 알려주세요:

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

또는 **자동 감지**로 둔 상태로 둘 수도 있습니다. 플랫폼 정보는 등록 시 LMS의 openid-configuration에서 읽어오며; 드롭다운은 생성된 구성에 표시될 레이블만 초기값으로 제공합니다.

#### URL 생성

**URL 생성**을 클릭합니다. FastComments는 일회성 등록 토큰을 생성하고 다음과 같은 URL을 표시합니다:

`https://fastcomments.com/lti/v1p3/register/<long-token>`

복사하세요. 이 URL은:

- **단회 사용**입니다 - LMS가 성공적으로 호출하면 토큰은 소모됩니다.
- 사용되지 않으면 **30분** 후에 만료됩니다.
- 비공개로 유지해야 합니다 - 이 URL을 가진 사람은 해당 30분 동안 테넌트에 도구를 등록할 수 있습니다.

#### 기존 구성

등록이 성공적으로 완료되면 새 구성은 동일한 페이지의 **기존 구성** 테이블에 플랫폼, Issuer, Client ID 및 상태와 함께 표시됩니다. 등록 해제가 필요한 경우 이 테이블에서 구성을 삭제할 수 있습니다.