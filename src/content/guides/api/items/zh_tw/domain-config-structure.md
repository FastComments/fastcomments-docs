A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = '網域設定結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 一個網域，不是 URL，例如 "fastcomments.com" 或 "www.example.com"。如果想限制到子網域，可以包含子網域。最多 1000 個字元。 **/
    domain: string
    /** 寄信時使用的寄件人名稱。 **/
    emailFromName?: string
    /** 寄信時使用的寄件人電子郵件。請確保 SPF 設定允許 mail.fastcomments.com 以此屬性所使用的網域發送郵件。 **/
    emailFromEmail?: string
    /** 唯讀。物件建立時間。 **/
    createdAt: string
    /** 與此網域相關的 logo。用於電子郵件。請使用 HTTPS。 **/
    logoSrc?: string
    /** 與此網域相關的較小 logo。請使用 HTTPS。 **/
    logoSrc100px?: string
    /** 僅 SSO。用於每封寄出的電子郵件頁尾的 URL。支援 "[userId]" 變數。 **/
    footerUnsubscribeURL?: string
    /** 僅 SSO。用於每封寄出電子郵件的標頭。例如可用來設定退訂相關的標頭以改善投遞率。此記錄中的 List-Unsubscribe 條目（如果存在）支援 "[userId]" 變數。 **/
    emailHeaders?: Record<string, string>
    /** 停用所有退訂連結。不建議，可能會降低投遞率。 **/
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
    /** 您的私鑰。以 -----BEGIN PRIVATE KEY----- 開頭，並以 -----END PRIVATE KEY----- 結尾。 **/
    privateKey: string
}
[inline-code-end]

### 用於驗證

網域設定用來決定哪些網站可以為您的帳戶承載 FastComments 小工具。這是一種基本的驗證方式，也就是說，新增或移除任何網域設定可能會影響您生產環境中 FastComments 安裝的可用性。

除非您打算停用該網域，否則不要移除或更新目前正在使用之網域的 `Domain Config` 中的 `domain` 屬性。

這與從 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) 中移除網域的行為相同。

另請注意，從 `My Domains` UI 移除網域也會刪除此介面中為該網域新增的任何對應設定。

### 用於電子郵件自訂

電子郵件頁尾的退訂連結，以及許多郵件用戶端提供的一鍵退訂功能，可以透過此 API 分別定義 `footerUnsubscribeURL` 與 `emailHeaders` 來設定。

### 用於 DKIM

在定義好 DKIM 的 DNS 紀錄後，只需使用上述結構將您的 DKIM 設定更新到 DomainConfig 即可。

---