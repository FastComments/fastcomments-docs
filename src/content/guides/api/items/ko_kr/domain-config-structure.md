`DomainConfig` 객체는 테넌트의 도메인에 대한 구성을 나타냅니다.

`DomainConfig` 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = '도메인 구성 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** "fastcomments.com" 또는 "www.example.com"과 같은 URL이 아닌 도메인입니다. 특정 서브도메인으로 제한하려는 경우 서브도메인을 포함할 수 있습니다. 최대 1000자. **/
    domain: string
    /** 이메일 발송 시 사용되는 From-Name입니다. **/
    emailFromName?: string
    /** 이메일 발송 시 사용되는 From-Email입니다. 이 속성에 사용된 도메인으로 mail.fastcomments.com이 이메일을 보낼 수 있도록 SPF가 설정되어 있는지 확인하십시오. **/
    emailFromEmail?: string
    /** 읽기 전용. 객체가 생성된 시점입니다. **/
    createdAt: string
    /** 이 도메인과 관련된 로고입니다. 이메일에 사용됩니다. HTTPS를 사용하십시오. **/
    logoSrc?: string
    /** 이 도메인과 관련된 더 작은 로고입니다. HTTPS를 사용하십시오. **/
    logoSrc100px?: string
    /** SSO 전용. 발송되는 모든 이메일의 푸터에 사용되는 URL입니다. "[userId]" 변수를 지원합니다. **/
    footerUnsubscribeURL?: string
    /** SSO 전용. 발송되는 모든 이메일에 사용되는 헤더입니다. 예를 들어 수신 거부 관련 헤더를 설정하여 전달률을 개선하는 데 유용합니다. 이 Record의 List-Unsubscribe 항목(존재하는 경우)은 "[userId]" 변수를 지원합니다. **/
    emailHeaders?: Record<string, string>
    /** 모든 수신 거부 링크를 비활성화합니다. 권장하지 않으며, 전달률에 악영향을 줄 수 있습니다. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM 구성입니다. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 구성 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM 레코드의 도메인 이름입니다. **/
    domainName: string
    /** 사용할 DKIM 키 셀렉터입니다. **/
    keySelector: string
    /** PEM 형식의 공개 키입니다. GET 응답에서 반환됩니다. **/
    publicKey: string
    /** @deprecated 더 이상 API 응답에서 반환되지 않습니다. 하위 호환성을 위해 쓰기 시 허용됩니다. **/
    privateKey?: string
}
[inline-code-end]

### 인증 관련

도메인 구성은 귀하의 계정에 대해 어떤 사이트가 FastComments 위젯을 호스팅할 수 있는지 결정하는 데 사용됩니다. 이는 기본적인 인증 형태로, 도메인 구성을 추가하거나 제거하면 프로덕션 환경에서 FastComments 설치의 가용성에 영향을 줄 수 있습니다.

해당 도메인을 비활성화하려는 의도가 없다면, 현재 사용 중인 도메인의 `Domain Config`에서 `domain` 속성을 제거하거나 업데이트하지 마십시오.

이는 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains)에서 도메인을 제거하는 것과 동일한 동작을 합니다.

또한 `My Domains` UI에서 도메인을 제거하면 이 UI를 통해 추가된 해당 도메인의 관련 구성도 함께 제거된다는 점에 유의하십시오.

### 이메일 사용자 정의 관련

이메일 푸터의 수신 거부 링크와 많은 이메일 클라이언트에서 제공하는 원클릭 수신 거부 기능은 각각 `footerUnsubscribeURL`과 `emailHeaders`를 정의하여 이 API를 통해 구성할 수 있습니다.

### DKIM 관련

DKIM DNS 레코드를 정의한 후, 정의된 구조를 사용하여 DKIM 구성으로 DomainConfig를 업데이트하기만 하면 됩니다.