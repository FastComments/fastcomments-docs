`DomainConfig` 物件代表租戶某個網域的設定。

`DomainConfig` 物件的結構如下：

[inline-code-attrs-start title = '網域設定結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 一個網域，而非 URL，例如 "fastcomments.com" 或 "www.example.com"。若希望限制為子網域，可包含子網域。最長 1000 個字元。 **/
    domain: string
    /** 發送電子郵件時使用的寄件者名稱（From-Name）。 **/
    emailFromName?: string
    /** 發送電子郵件時使用的寄件者電子郵件（From-Email）。請確保已設定 SPF，允許 mail.fastcomments.com 以此屬性中使用的網域發送電子郵件。 **/
    emailFromEmail?: string
    /** 唯讀。物件建立時間。 **/
    createdAt: string
    /** 與此網域相關的標誌（logo）。用於電子郵件中。請使用 HTTPS。 **/
    logoSrc?: string
    /** 與此網域相關的較小標誌。請使用 HTTPS。 **/
    logoSrc100px?: string
    /** 僅限 SSO。用於每封發送電子郵件頁尾的 URL。支援 "[userId]" 變數。 **/
    footerUnsubscribeURL?: string
    /** 僅限 SSO。用於每封發送電子郵件的標頭。例如可用於設定與取消訂閱相關的標頭以改善送達率。此 Record 中的 List-Unsubscribe 項目（若存在）支援 "[userId]" 變數。 **/
    emailHeaders?: Record<string, string>
    /** 停用所有取消訂閱連結。不建議這樣做，可能會影響送達率。 **/
    disableUnsubscribeLinks?: boolean
    /** DKIM 設定。 **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 設定結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** 您 DKIM 記錄中的網域名稱。 **/
    domainName: string
    /** 要使用的 DKIM 金鑰選擇器（key selector）。 **/
    keySelector: string
    /** 公鑰，PEM 格式。會在 GET 回應中傳回。 **/
    publicKey: string
    /** @deprecated 不再於 API 回應中傳回。為向後相容，寫入時仍可接受。 **/
    privateKey?: string
}
[inline-code-end]

### 用於驗證

網域設定用於決定哪些網站可以為您的帳戶託管 FastComments 小工具。這是一種基本的驗證形式，這意味著新增或移除任何網域設定都可能會影響您在正式環境中 FastComments 安裝的可用性。

除非您有意停用某個網域，否則請勿移除或更新目前正在使用之網域的 `Domain Config` 的 `domain` 屬性。

此行為與從 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) 移除網域相同。

另請注意，從 `My Domains` 介面移除網域時，也會一併移除透過此介面新增的該網域的任何對應設定。

### 用於電子郵件自訂

電子郵件頁尾中的取消訂閱連結，以及許多電子郵件用戶端提供的一鍵取消訂閱功能，可分別透過定義 `footerUnsubscribeURL` 和 `emailHeaders` 來經由此 API 進行設定。

### 用於 DKIM

在定義您的 DKIM DNS 記錄之後，只需使用已定義的結構，以您的 DKIM 設定更新 DomainConfig 即可。