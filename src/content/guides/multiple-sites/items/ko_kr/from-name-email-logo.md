---
때때로 FastComments는 특히 Secure SSO를 사용하지 않는 경우 사용자에게 이메일을 보내야 할 때가 있습니다.

예로는 계정 확인 또는 처음 댓글을 달 때의 활동 확인이 있습니다. FastComments
는 또한 댓글에 대한 응답 알림도 보냅니다.

When FastComments emails your users, we will use a default From Name and Email of `FastComments Robot` and `noreply@fastcomments.com`.

또한 이러한 이메일의 하단에는 당사의 로고가 사용됩니다.

If you have FastComments Flex or Pro, this all can be customized on a per-domain basis via the "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

이메일에 표시되는 로고를 사용자화할 때, 업로드하는 이미지의 크기가 이메일 하단에 표시하려는 크기와 동일한지 확인하십시오.

### `From Domain`을(를) 사용자 지정할 때

`From Domain`을 사용자 지정하면, 이메일 제공업체와 클라이언트가 FastComments가 귀하를 대신해 이메일을 보낼 권한이 있음을 알아야 합니다. 그렇지 않으면,
`From Domain`을 정의하고 아래 단계를 따르지 않으면 이메일이 스팸으로 이동할 가능성이 높습니다.

#### 1. SPF 설정

FastComments가 귀하의 도메인으로 안전하게 이메일을 보낼 수 있도록 허용하려면, 이를 허용하는 SPF 레코드를 추가했는지 확인하십시오.

`mail.fastcomments.com` 및 `sib.fastcomments.com`이 귀하의 도메인으로 메일을 보낼 수 있도록 허용하는 SPF 레코드가 있는지 확인하십시오.

이 작업을 수행하는 방법에 대한 추가 정보는 다음을 참조하십시오: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM 설정

SPF 외에도 DKIM을 설정해야 합니다. DNS 구성이 준비되면 도메인 구성 페이지에서 "Show Advanced"를 클릭하여
도메인별 DKIM 설정을 표시할 수 있습니다.

또한 DKIM 구성을 설정하기 위해 [invoke the API](/guide-api.html#domain-config-structure)할 수도 있습니다.

### 구독 취소 링크

SSO를 사용하는 경우, 이메일 및 알림에 사용되는 구독 취소 기능은 [via the DomainConfigs API](/guide-api.html#domain-config-structure)에서 사용자화할 수 있습니다.

---