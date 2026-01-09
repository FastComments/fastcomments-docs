A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = '도메인 구성 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 도메인입니다. URL이 아니라 "fastcomments.com" 또는 "www.example.com" 같은 형식입니다. 서브도메인으로 제한하려면 서브도메인을 포함할 수 있습니다. 최대 1000자. **/
    domain: string
    /** 이메일 발송 시 사용되는 발신자 이름(From-Name). **/
    emailFromName?: string
    /** 이메일 발송 시 사용되는 발신자 이메일(From-Email). 이 속성에 사용된 도메인으로 mail.fastcomments.com이 이메일을 보낼 수 있도록 SPF가 설정되어 있는지 확인하세요. **/
    emailFromEmail?: string
    /** 읽기 전용. 객체가 생성된 시각. **/
    createdAt: string
    /** 이 도메인과 관련된 로고. 이메일에 사용됩니다. HTTPS를 사용하세요. **/
    logoSrc?: string
    /** 이 도메인과 관련된 더 작은 로고. HTTPS를 사용하세요. **/
    logoSrc100px?: string
    /** SSO 전용. 전송되는 모든 이메일의 푸터에 사용되는 URL입니다. "[userId]" 변수를 지원합니다. **/
    footerUnsubscribeURL?: string
    /** SSO 전용. 전송되는 모든 이메일에 사용되는 헤더입니다. 예를 들어 배달 개선을 위해 구독취소 관련 헤더를 설정할 때 유용합니다. 이 레코드의 List-Unsubscribe 항목이 존재하는 경우 "[userId]" 변수를 지원합니다. **/
    emailHeaders?: Record<string, string>
    /** 모든 구독취소 링크를 비활성화합니다. 권장되지 않으며, 배달률에 악영향을 줄 수 있습니다. **/
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
    /** 사용될 DKIM 키 셀렉터. **/
    keySelector: string
    /** 개인 키입니다. -----BEGIN PRIVATE KEY-----로 시작하고 -----END PRIVATE KEY-----로 끝나야 합니다. **/
    privateKey: string
}
[inline-code-end]

### 인증을 위해

도메인 구성은 귀하의 계정에 대해 어떤 사이트들이 FastComments 위젯을 호스팅할 수 있는지를 결정하는 데 사용됩니다. 이는 기본적인 형태의
인증이며, 도메인 구성을 추가하거나 제거하면 운영 환경에서 FastComments 설치의 가용성에 영향을 줄 수 있습니다.

현재 사용 중인 도메인에 대해 해당 도메인의 `Domain Config`에서 `domain` 속성을 의도적으로 비활성화하지 않는 한 제거하거나 업데이트하지 마십시오.

이것은 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains)에서 도메인을 제거하는 것과 동일한 동작을 합니다.

또한 `My Domains` UI에서 도메인을 제거하면 이 UI를 통해 추가되었을 수 있는 해당 도메인에 대한 모든 구성도 제거된다는 점에 유의하십시오.

### 이메일 사용자화

이메일 푸터의 구독취소 링크 및 많은 이메일 클라이언트가 제공하는 원클릭 구독취소 기능은 각각 `footerUnsubscribeURL`과 `emailHeaders`를 정의하여 이 API를 통해 구성할 수 있습니다.

### DKIM

DKIM DNS 레코드를 정의한 후, 위에서 정의한 구조를 사용하여 도메인 구성에 DKIM 구성을 업데이트하면 됩니다.

---