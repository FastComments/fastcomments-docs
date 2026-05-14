#### "Registration token not found, expired, or already used"

등록 URL의 토큰은 30분 동안만 유효하며 한 번만 사용할 수 있습니다. LMS가 그보다 오래 걸렸거나 등록이 성공한 후 다시 시도된 경우 토큰이 거부됩니다. FastComments LTI 1.3 Configuration 페이지에서 새 URL을 생성하고 다시 시작하세요.

#### "Platform rejected registration"

귀하의 LMS가 등록 핸드셰이크를 거부했습니다. 가장 흔한 원인:

- **Tool already registered with the same client name.** 일부 플랫폼(특히 D2L)은 이전 항목이 삭제될 때까지 "FastComments"의 두 번째 등록을 거부합니다. LMS에서 기존 도구를 제거한 다음 다시 시도하세요.
- **Wrong field in the LMS.** URL을 런치 URL이나 로그인 URL 필드가 아니라 **registration / tool initiation registration endpoint** 필드에 붙여넣었는지 확인하세요.
- **The LMS doesn't actually support Dynamic Registration.** 구버전의 Moodle 및 Blackboard는 LTI 1.3을 표기하지만 수동 구성만 허용합니다. 플랫폼 문서를 확인하세요.

#### "Failed to fetch platform configuration"

FastComments가 귀하의 LMS의 openid-configuration 문서를 읽을 수 없었습니다. 이는 드물며 보통 LMS가 잘못된 형식이거나 접근할 수 없는 discovery URL을 제공했음을 의미합니다. LMS 지원팀에 문의하세요.

#### Launch shows "Configuration not found"

FastComments의 구성이 삭제되었거나, 런치가 우리가 인식하지 못하는 `iss`/`client_id` 쌍에서 왔습니다. 삭제 후 재등록을 한 경우, LMS에 FastComments 도구를 제거한 뒤 다시 추가해 새 `client_id`를 받도록 하세요.

#### Launch shows "Deployment not registered"

처음 런치했던 배포와 다른 Brightspace/Moodle/Blackboard 배포에서 FastComments를 시작했습니다. FastComments는 첫 런치 시 `deployment_id`를 보안 검사용으로 고정합니다. 동일한 클라이언트 아래에 새 배포를 추가하려면 지원팀에 문의하세요 — 배포 ID를 구성에 추가해 드립니다.

#### Launch shows "Unsupported message_type"

LMS가 FastComments가 처리하지 않는 LTI 메시지(예: `LtiSubmissionReviewRequest`)를 보냈습니다. FastComments는 표준 resource-link 런치와 deep-linking 흐름만 지원합니다. 특정 메시지 유형 추가가 필요하시면 문의하세요.

#### Iframe doesn't resize

대부분의 LMS는 LTI iframe의 크기를 자동으로 조정합니다. 귀하의 LMS가 그렇지 않다면, LMS의 런치 설정이 도구가 부모 프레임으로 postMessage 이벤트를 보낼 수 있도록 허용하는지 확인하세요. FastComments는 Canvas 스타일(`lti.frameResize`)과 IMS 규격(`org.imsglobal.lti.frameResize`)의 리사이즈 메시지 둘 다 전송합니다.