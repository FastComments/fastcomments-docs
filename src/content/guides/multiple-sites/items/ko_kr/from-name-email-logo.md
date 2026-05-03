때때로 FastComments는 사용자의 이메일로 메시지를 보내야 합니다. 특히 Secure SSO를 사용하지 않는 경우에 그렇습니다.

예시로는 계정 확인이나 사용자가 처음으로 댓글을 달 때 활동을 확인하는 이메일이 있습니다. FastComments는 또한 댓글에 대한 답글 알림을 보냅니다.

FastComments가 사용자에게 이메일을 보낼 때 기본 From 이름과 이메일로 `FastComments Robot` 및 `noreply@fastcomments.com`을(를) 사용합니다.

또한 이러한 이메일의 푸터에 당사의 로고를 사용합니다.

FastComments Flex 또는 Pro를 사용 중인 경우, 이는 도메인별로 "My Domains page"에서 사용자 지정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

이메일에 표시되는 로고를 사용자 지정할 때는 업로드하는 이미지의 크기가 이메일 푸터에 표시하려는 크기와 동일한지 확인하세요.

### `From Domain` 사용자 지정 시

`From Domain`을 사용자 지정하면 이메일 제공업체와 클라이언트는 FastComments가 귀하를 대신하여 이메일을 보낼 권한이 있음을 알아야 합니다. 그렇지 않으면 `From Domain`을 정의하고 아래 단계를 따르지 않을 경우 이메일이 스팸으로 처리될 가능성이 높습니다.

#### 1. SPF 설정

FastComments가 귀하의 도메인으로 안전하게 이메일을 보낼 수 있도록 허용하려면, 이를 허용하는 SPF 레코드를 추가했는지 확인하세요.

`mail.fastcomments.com` 및 `sib.fastcomments.com`이 귀하의 도메인으로 메일을 보낼 수 있도록 허용하는 SPF 레코드가 존재하는지 확인하세요.

이 작업에 대한 추가 정보는 다음을 참조하세요: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM 설정

SPF 외에도 DKIM을 설정해야 합니다. DNS 구성이 준비되면 도메인 구성 페이지에서 "고급 표시"를 클릭하여 도메인별 DKIM 설정을 표시할 수 있습니다.

DKIM 구성을 설정하려면 [API 호출](/guide-api.html#domain-config-structure)를 사용할 수도 있습니다.

### 수신 거부 링크

SSO를 사용하는 경우, 이메일 및 알림에서 사용되는 수신 거부 기능은 [DomainConfigs API](/guide-api.html#domain-config-structure)를 통해 사용자 지정할 수 있습니다.

### 이메일 링크 난독화

사이트의 도메인 평판 때문에 알림 이메일이 스팸으로 분류되는 경우, 페이지로 직접 연결하는 대신 `fastcomments.com`을 통해 "view comment" 버튼을 라우팅할 수 있습니다. 메일박스 제공업체는 이메일 본문에 있는 모든 링크를 목적지 평판과 비교하여 점수를 매기므로, 도메인이 플래그되어 있는 경우 링크 자체가 발송 설정이 얼마나 정상이든 상관없이 스팸 점수에 기여합니다.

이 설정은 My Domains 페이지의 "고급 표시"에서 "Email Link Obfuscation" 섹션으로 이동하여 활성화할 수 있습니다. 설정은 도메인별로 적용됩니다.

활성화하면 mention, reply, new-comment, subscribed-page, profile-comment, 및 digest 이메일의 링크가 클릭 시 원래 페이지로 리디렉션되는 짧은 토큰으로 재작성됩니다. 목적지는 귀하의 테넌트에 묶여 있습니다: 리디렉션은 구성된 도메인 중 하나와 호스트가 일치하는 URL로만 전달되며, 토큰은 30일 후 자동 만료됩니다.

클릭하여 전달되는 경험은 변경되지 않습니다. 읽는 사람은 여전히 댓글이 보이도록 스크롤된 상태로 귀하의 페이지에 도달합니다.