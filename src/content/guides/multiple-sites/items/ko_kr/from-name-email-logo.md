---
때때로 FastComments는 사용자가 Secure SSO를 사용하지 않을 경우 이메일을 보내야 합니다.

예를 들어, 처음 댓글을 달 때 계정이나 활동을 확인하는 경우가 있습니다. FastComments는 댓글에 대한 답글 알림도 보냅니다.

FastComments가 사용자에게 이메일을 보낼 때, 기본 발신자 이름과 이메일은 `FastComments Robot` 및 `noreply@fastcomments.com`을 사용합니다.

또한 이러한 이메일의 하단에 자체 로고를 사용합니다.

FastComments Flex 또는 Pro를 사용 중이라면, 모든 설정을 “My Domains 페이지”에서 도메인별로 맞춤 설정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; alt='도메인별 이메일 설정 양식(보내는 사람 이름, 보내는 사람 이메일 및 로고 업로드 필드 포함)'; title='보내는 사람 이름, 이메일 및 로고 사용자 지정' app-screenshot-end]

이메일에 표시되는 로고를 맞춤 설정할 때, 업로드하는 크기가 이메일 하단에 표시하려는 크기와 동일한지 확인하십시오.

### `From Domain` 맞춤 설정 시

`From Domain`을 맞춤 설정하면, 이메일 제공업체와 클라이언트가 FastComments가 귀하를 대신해 이메일을 보낼 권한이 있음을 알아야 합니다. 그렇지 않으면,
`From Domain`을 정의하고 아래 단계를 따르지 않으면 이메일이 스팸으로 전송될 가능성이 높습니다.

#### 1. SPF 설정

FastComments가 귀하의 도메인으로 안전하게 이메일을 보낼 수 있도록, 이를 허용하는 SPF 레코드를 추가하십시오.

`mail.fastcomments.com` 및 `sib.fastcomments.com`이 귀하의 도메인으로 메일을 보낼 수 있도록 SPF 레코드가 있는지 확인하십시오.

이와 관련된 자세한 내용은 여기에서 확인할 수 있습니다: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM 설정

SPF 외에도 DKIM을 설정해야 합니다. DNS 구성이 완료되면 도메인 설정 페이지에서 “Show Advanced”를 클릭하여 도메인별 DKIM 설정을 확인할 수 있습니다.

또한 [API 호출](/guide-api.html#domain-config-structure)로 DKIM 구성을 설정할 수 있습니다.

### 구독 해지 링크

SSO를 사용할 때, 이메일 및 알림에 사용되는 구독 해지 기능은 [DomainConfigs API](/guide-api.html#domain-config-structure)를 통해 맞춤 설정할 수 있습니다.

### 이메일 링크 난독화

사이트 도메인 평판 때문에 알림 이메일이 스팸으로 분류되는 경우, “view comment” 버튼을 직접 페이지에 연결하는 대신 `fastcomments.com`을 통해 라우팅할 수 있습니다. 메일함 제공업체는 이메일 본문의 모든 링크를 대상의 평판과 비교해 점수를 매기므로, 도메인이 플래그될 경우 직접 링크가 스팸 점수에 영향을 줍니다.

“My Domains 페이지”의 “Show Advanced” 아래 “Email Link Obfuscation” 섹션에서 이 기능을 활성화하십시오. 설정은 도메인별로 적용됩니다.

활성화하면, 멘언션, 답글, 새 댓글, 구독 페이지, 프로필 댓글 및 다이제스트 이메일의 링크가 짧은 토큰으로 재작성되어 클릭 시 원본 페이지로 리다이렉트됩니다. 대상은 귀하의 테넌트에 바인딩되며, 리다이렉트는 구성된 도메인 중 하나와 호스트가 일치하는 URL로만 전달되고 토큰은 30일 후 자동 만료됩니다.

클릭 후 경험은 변하지 않습니다. 독자는 여전히 댓글이 보이는 위치로 스크롤된 페이지에 도착합니다.
---