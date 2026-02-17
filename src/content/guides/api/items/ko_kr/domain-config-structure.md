A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = '도메인 구성 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 도메인(예: "fastcomments.com" 또는 "www.example.com"). URL이 아닙니다. 서브도메인으로 제한하려는 경우 서브도메인을 포함할 수 있습니다. 최대 1000자. **/
    domain: string
    /** 이메일 발송 시 사용되는 발신자 이름(From-Name). **/
    emailFromName?: string
    /** 이메일 발송 시 사용되는 발신자 이메일(From-Email). 이 속성에 사용된 도메인으로 mail.fastcomments.com이 이메일을 보낼 수 있도록 SPF가 설정되어 있는지 확인하세요. **/
    emailFromEmail?: string
    /** 읽기 전용. 객체가 생성된 시각. **/
    createdAt: string
    /** 이 도메인과 관련된 로고. 이메일에 사용됩니다. HTTPS를 사용하세요. **/
    logoSrc?: string
    /** 이 도메인과 관련된 작은 로고. HTTPS를 사용하세요. **/
    logoSrc100px?: string
    /** SSO 전용. 발송되는 모든 이메일의 하단(푸터)에 사용되는 URL입니다. "[userId]" 변수를 지원합니다. **/
    footerUnsubscribeURL?: string
    /** SSO 전용. 발송되는 모든 이메일에 사용되는 헤더입니다. 예를 들어 수신거부 관련 헤더를 설정하여 전달률을 개선하는 데 유용합니다. 이 레코드의 List-Unsubscribe 항목이 존재하면 "[userId]" 변수를 지원합니다. **/
    emailHeaders?: Record<string, string>
    /** 모든 수신거부(언서브스크라이브) 링크를 비활성화합니다. 권장되지 않으며 전달률에 악영향을 미칠 수 있습니다. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM 구성. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 구성 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM 레코드에 사용되는 도메인 이름. **/
    domainName: string
    /** 사용할 DKIM 키 선택자. **/
    keySelector: string
    /** PEM 형식의 공개 키. GET 응답에서 반환됩니다. **/
    publicKey: string
    /** @deprecated 더 이상 API 응답에서 반환되지 않습니다. 하위 호환성을 위해 쓰기 시에는 허용됩니다. **/
    privateKey?: string
}
[inline-code-end]

### 인증을 위해

도메인 구성은 귀하의 계정에 대해 FastComments 위젯을 호스팅할 수 있는 사이트를 결정하는 데 사용됩니다. 이것은 기본적인 형태의 인증으로, 도메인 구성을 추가하거나 제거하면 운영 환경의 FastComments 설치 가용성에 영향을 줄 수 있습니다.

현재 사용 중인 도메인의 `Domain Config`에서 `domain` 속성을 비활성화하려는 의도가 아닌 한 제거하거나 업데이트하지 마세요.

이 동작은 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains)에서 도메인을 제거하는 것과 동일합니다.

또한 `My Domains` UI에서 도메인을 제거하면 해당 UI를 통해 추가되었을 수 있는 해당 도메인에 대한 구성도 제거된다는 점을 유의하세요.

### 이메일 커스터마이징을 위해

이메일 푸터의 수신거부 링크와 많은 이메일 클라이언트에서 제공하는 원클릭 수신거부 기능은 각각 `footerUnsubscribeURL` 및 `emailHeaders`를 정의하여 이 API를 통해 구성할 수 있습니다.

### DKIM을 위해

DKIM DNS 레코드를 정의한 후, 정의된 구조를 사용하여 DomainConfig를 DKIM 구성으로 업데이트하면 됩니다.