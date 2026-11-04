D2L Brightspace는 LTI Advantage 관리자 인터페이스를 통해 Dynamic Registration을 제공합니다. 관리자 권한이 필요합니다.

#### 등록 화면 열기

1. 관리자로 Brightspace 인스턴스에 로그인합니다.
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage**로 이동합니다.
3. **Register Tool**을 클릭합니다. (직접 URL은 `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`입니다.)

#### URL 붙여넣기

등록 양식이 표시됩니다. 핵심 필드는 **Tool initiation registration endpoint**입니다 (일부 Brightspace 버전에서는 "Tool Initiation Registration URL"로 표시됩니다).

FastComments 등록 URL(<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">여기서 받으세요</a>)을 해당 필드에 붙여넣습니다. 다른 필드는 비워 두세요. 등록 핸드셰이크 과정에서 FastComments가 자동으로 채워 넣습니다.

**Register**를 클릭합니다.

#### 도구 승인

Brightspace가 FastComments와 통신하고 키를 교환하며 확인 화면을 표시하는 팝업을 엽니다. 등록이 완료되면 팝업이 자동으로 닫힙니다.

새 도구가 LTI Advantage 도구 목록에 나타납니다. 기본적으로 Brightspace는 새 도구를 **비활성화** 상태로 표시하므로, 강좌에서 사용할 수 있도록 토글을 **활성화**로 전환하세요.

#### 배포 추가

Brightspace에서 LTI 도구는 강좌에서 사용되기 전에 **배포(deployment)**가 필요합니다:

1. 새로 등록된 FastComments 도구를 엽니다.
2. **View Deployments** > **New Deployment**를 클릭합니다.
3. 배포 이름(예: "FastComments - All Courses")을 지정하고, 사용 가능한 조직 단위를 선택한 후 저장합니다.

이 배포를 통한 첫 실행 이후, FastComments는 `deployment_id`를 구성 레코드에 고정합니다. 동일한 클라이언트에서 다른 배포를 통한 이후의 실행은 재등록하지 않는 한 거부됩니다.