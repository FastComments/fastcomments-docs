D2L Brightspace는 LTI Advantage 관리 인터페이스를 통해 Dynamic Registration을 제공합니다. 관리자 권한이 필요합니다.

#### Open the Registration Screen

1. 관리자로 Brightspace 인스턴스에 로그인합니다.
2. **Admin Tools** > **Manage Extensibility** > **LTI Advantage**로 이동합니다.
3. **Register Tool**을 클릭합니다. (직접 URL은 `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create` 입니다.)

#### Paste the URL

등록 양식이 표시됩니다. 핵심 필드는 **Tool initiation registration endpoint** (일부 Brightspace 버전에서는 "Tool Initiation Registration URL"로 표시됨)입니다.

해당 필드에 FastComments 등록 URL을 붙여넣으세요. 다른 필드는 비워 두세요 - 등록 핸드셰이크 동안 FastComments가 자동으로 채웁니다.

**Register**를 클릭합니다.

#### Approve the Tool

Brightspace가 FastComments와 통신하고 키를 교환하며 확인 화면을 표시하는 팝업을 엽니다. 등록이 완료되면 팝업은 자동으로 닫힙니다.

새 도구가 LTI Advantage 도구 목록에 표시됩니다. 기본적으로 Brightspace는 새 도구를 **disabled**로 표시하므로 토글을 **enabled**로 전환하여 코스에서 사용할 수 있게 하세요.

#### Add a Deployment

Brightspace에서는 LTI 도구가 코스에서 사용되기 전에 **deployment**가 필요합니다:

1. 새로 등록된 FastComments 도구를 엽니다.
2. **View Deployments** > **New Deployment**를 클릭합니다.
3. 배포에 이름을 지정하세요(예: "FastComments - All Courses"), 해당 배포를 적용할 조직 단위를 선택하고 저장합니다.

이 배포를 통해 처음 실행한 이후 FastComments는 `deployment_id`를 구성 레코드에 고정합니다 - 같은 클라이언트 아래의 다른 배포에서 이후에 실행하면 재등록하지 않는 한 거부됩니다.