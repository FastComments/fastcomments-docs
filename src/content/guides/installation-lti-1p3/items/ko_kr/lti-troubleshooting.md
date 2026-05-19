#### "등록 토큰을 찾을 수 없거나, 만료되었거나, 이미 사용됨"

등록 URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기에서 확인하세요</a>)의 토큰은 30분 동안만 유효하며 한 번만 사용할 수 있습니다. LMS에서 그보다 오래 걸렸거나 등록이 성공한 뒤 재시도된 경우 토큰이 거부됩니다. FastComments LTI 1.3 구성 페이지에서 새 URL을 생성하고 처음부터 다시 시작하세요.

#### "플랫폼이 등록을 거부함"

LMS가 등록 핸드셰이크를 거부했습니다. 가장 흔한 원인:

- **동일한 클라이언트 이름으로 이미 도구가 등록되어 있음.** 일부 플랫폼(특히 D2L)은 이전 항목을 삭제할 때까지 "FastComments"를 두 번째로 등록하는 것을 거부합니다. LMS에서 이전 도구를 제거한 다음 다시 시도하세요.
- **LMS의 잘못된 필드.** URL을 런치 URL이나 로그인 URL 필드가 아닌 **registration / tool initiation registration endpoint** 필드에 붙여넣었는지 확인하세요.
- **LMS가 실제로 Dynamic Registration을 지원하지 않음.** 이전 버전의 Moodle 및 Blackboard는 LTI 1.3을 광고하지만 수동 구성만 허용합니다. 플랫폼 설명서를 확인하세요.

#### "플랫폼 구성 가져오기 실패"

FastComments가 LMS의 openid-configuration 문서를 읽지 못했습니다. 이는 드물며 일반적으로 LMS가 잘못되었거나 접근할 수 없는 discovery URL을 제공했음을 의미합니다. LMS 지원팀에 문의하세요.

#### 런치에 "구성을 찾을 수 없음" 표시

FastComments의 구성이 삭제되었거나, 런치가 우리가 인식하지 못하는 `iss`/`client_id` 쌍에서 왔을 수 있습니다. 삭제 후 다시 등록했다면, LMS에 FastComments 도구를 제거했다가 다시 추가하도록 지시하여 새 client_id를 받도록 하세요.

#### 런치에 "배포가 등록되지 않음" 표시

FastComments를 처음 런치한 것과 다른 Brightspace/Moodle/Blackboard 배포에서 FastComments를 실행했습니다. FastComments는 첫 런치 시 보안 확인을 위해 `deployment_id`를 고정합니다. 동일한 클라이언트 아래에 새 배포를 추가하려면 지원팀에 문의하세요 — 저희가 구성에 해당 deployment ID를 추가해 드립니다.

#### 런치에 "지원되지 않는 message_type" 표시

LMS가 FastComments가 처리하지 않는 LTI 메시지(예: `LtiSubmissionReviewRequest`)를 보냈습니다. FastComments는 표준 resource-link 런치 및 deep-linking 흐름만 지원합니다. 특정 메시지 타입 추가가 필요하면 문의하세요.

#### Iframe 크기 조정이 되지 않음

대부분의 LMS는 LTI iframe을 자동으로 크기 조정합니다. 귀하의 LMS가 자동 조정하지 않는 경우, LMS의 런치 설정이 도구가 부모 프레임으로 postMessage 이벤트를 보낼 수 있도록 허용하는지 확인하세요. FastComments는 Canvas-style (`lti.frameResize`) 및 IMS-spec (`org.imsglobal.lti.frameResize`) 리사이즈 메시지 둘 다를 전송합니다.