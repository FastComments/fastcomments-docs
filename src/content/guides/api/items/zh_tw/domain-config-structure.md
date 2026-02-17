A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = '網域設定結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 一個網域，不是 URL，例如 "fastcomments.com" 或 "www.example.com"。如果想限定子網域，也可以包含子網域。最多 1000 字元。 **/
    domain: string
    /** 用於發送電子郵件的寄件人名稱。 **/
    emailFromName?: string
    /** 用於發送電子郵件的寄件人電子郵件。確保已設定 SPF 以允許 mail.fastcomments.com 代表此屬性所使用的網域寄送電子郵件。 **/
    emailFromEmail?: string
    /** 唯讀。物件建立時間。 **/
    createdAt: string
    /** 此網域相關的 logo。用於電子郵件。請使用 HTTPS。 **/
    logoSrc?: string
    /** 此網域相關的小尺寸 logo。請使用 HTTPS。 **/
    logoSrc100px?: string
    /** 僅限 SSO。用於每封寄出電子郵件頁尾的 URL。支援 "[userId]" 變數。 **/
    footerUnsubscribeURL?: string
    /** 僅限 SSO。用於每封寄出電子郵件的標頭。此設定可用於例如設定取消訂閱相關的標頭以改善送達率。此記錄中的 List-Unsubscribe 條目（若存在）支援 "[userId]" 變數。 **/
    emailHeaders?: Record<string, string>
    /** 停用所有取消訂閱連結。不建議，可能會影響郵件送達率。 **/
    disableUnsubscribeLinks?: boolean
    /** DKIM 設定。 **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 設定結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM 紀錄中的網域名稱。 **/
    domainName: string
    /** 要使用的 DKIM 金鑰選擇器。 **/
    keySelector: string
    /** 公開金鑰，PEM 格式。在 GET 回應中會回傳。 **/
    publicKey: string
    /** @deprecated 不再於 API 回應中回傳。為了向下相容，寫入時仍接受。 **/
    privateKey?: string
}
[inline-code-end]

### For Authentication

Domain Configuration is used to determine which sites can host the FastComments widget for your account. This is a basic form
of authentication, meaning adding or removing any Domain Configurations can impact the availability of your FastComments installation
in production.

Don't remove or update the `domain` property of a `Domain Config` for a domain that is currently in use unless disabling that domain is intended.

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

Also note that removing a domain from the `My Domains` UI will remove any corresponding configuration for that domain that may have been added via this UI.

### For Email Customization

The unsubscribe link in the email footer, and the one-click-unsubscribe feature offered by many email clients, can be configured via this API by defining `footerUnsubscribeURL` and `emailHeaders`, respectively.

### For DKIM

After defining your DKIM DNS records, simply update the DomainConfig with your DKIM configuration using the defined structure.