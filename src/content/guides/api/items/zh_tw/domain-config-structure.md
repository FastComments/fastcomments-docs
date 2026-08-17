A `DomainConfig` 物件代表租戶的網域設定。

`DomainConfig` 物件的結構如下：

[inline-code-attrs-start title = 'Domain Config 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** A domain, not a URL, like "fastcomments.com" or "www.example.com". Subdomain may be included if limiting to a subdomain is desired. Max 1000 characters. **/
    domain: string
    /** The From-Name used when sending emails. **/
    emailFromName?: string
    /** The From-Email used when sending emails. Ensure SPF is setup to allow mail.fastcomments.com to send emails as the domain used in this attribute. **/
    emailFromEmail?: string
    /** READONLY. When the object was created. **/
    createdAt: string
    /** The logo related to this domain. Used in emails. Use HTTPS. **/
    logoSrc?: string
    /** A smaller logo related to this domain. Use HTTPS. **/
    logoSrc100px?: string
    /** SSO ONLY. The URL used in the footer of every email sent. Supports a "[userId]" variable. **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. The headers used in of every email sent. Useful for example for setting unsubscribe related headers to improve delivery. The List-Unsubscribe entry in this Record, if it exists, supports a "[userId]" variable. **/
    emailHeaders?: Record<string, string>
    /** Disable all unsubscribe links. Not recommended, may hurt delivery rates. **/
    disableUnsubscribeLinks?: boolean
    /** DKIM Configuration. **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM Config 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** The domain name in your DKIM record. **/
    domainName: string
    /** The DKIM key selector to use. **/
    keySelector: string
    /** The public key, in PEM format. Returned in GET responses. **/
    publicKey: string
    /** @deprecated No longer returned in API responses. Accepted on write for backwards compatibility. **/
    privateKey?: string
}
[inline-code-end]

### For Authentication

Domain Configuration 用於決定哪些網站可以為您的帳號託管 FastComments 小工具。這是一種基本的驗證形式，意味著新增或移除任何 Domain Configurations 都可能影響您在正式環境中 FastComments 安裝的可用性。

除非確實要停用該網域，否則不要移除或更新 `Domain Config` 中的 `domain` 屬性，尤其是該網域目前仍在使用中。

此行為與從 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) 移除網域的行為相同。

另請注意，從 `My Domains` 介面中移除網域，會同時刪除透過此 UI 新增的任何相對應設定。

### For Email Customization

電子郵件底部的退訂連結，以及許多郵件客戶端提供的一鍵退訂功能，都可以透過此 API 設定，分別使用 `footerUnsubscribeURL` 與 `emailHeaders`。

### For DKIM

在定義好 DKIM DNS 記錄後，只需使用上述結構將 DKIM 設定更新至 DomainConfig 即可。