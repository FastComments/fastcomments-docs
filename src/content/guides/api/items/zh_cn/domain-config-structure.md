A `DomainConfig` object represents configuration for a domain for a tenant.

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = '域配置结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 一个域名，不是 URL，例如 "fastcomments.com" 或 "www.example.com"。如果希望限制到子域，可以包含子域。最大 1000 个字符。 **/
    domain: string
    /** 发送电子邮件时使用的发件人姓名。 **/
    emailFromName?: string
    /** 发送电子邮件时使用的发件人邮箱。确保已设置 SPF，允许 mail.fastcomments.com 以此属性中使用的域发送邮件。 **/
    emailFromEmail?: string
    /** 只读。对象创建时间。 **/
    createdAt: string
    /** 与此域相关的徽标。用于电子邮件。请使用 HTTPS。 **/
    logoSrc?: string
    /** 与此域相关的较小徽标。请使用 HTTPS。 **/
    logoSrc100px?: string
    /** 仅限 SSO。用于每封发送的电子邮件页脚的 URL。支持 "[userId]" 变量。 **/
    footerUnsubscribeURL?: string
    /** 仅限 SSO。用于每封发送的电子邮件的头信息。例如可用于设置与退订相关的头以改善投递。此记录中的 List-Unsubscribe 条目（如果存在）支持 "[userId]" 变量。 **/
    emailHeaders?: Record<string, string>
    /** 禁用所有退订链接。不建议，可能会影响投递率。 **/
    disableUnsubscribeLinks?: boolean
    /** DKIM 配置。 **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 配置结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM 记录中使用的域名。 **/
    domainName: string
    /** 要使用的 DKIM 密钥选择器。 **/
    keySelector: string
    /** 您的私钥。以 -----BEGIN PRIVATE KEY----- 开始，以 -----END PRIVATE KEY----- 结束 **/
    privateKey: string
}
[inline-code-end]

### 用于身份验证

域配置用于确定哪些站点可以为您的账户托管 FastComments 小部件。这是一种基本形式
的身份验证，意味着添加或删除任何域配置都可能影响您在生产环境中 FastComments 安装的可用性。

不要删除或更新当前正在使用的域的 `Domain Config` 的 `domain` 属性，除非您确实打算禁用该域。

这与从 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) 中移除域具有相同的行为。

另请注意，从 `My Domains` 界面移除域将删除通过该界面为该域添加的任何对应配置。

### 用于电子邮件自定义

电子邮件页脚中的退订链接以及许多邮件客户端提供的一键退订功能，可以通过此 API 分别通过定义 `footerUnsubscribeURL` 和 `emailHeaders` 来配置。

### 用于 DKIM

在定义好 DKIM DNS 记录后，只需使用前述结构将您的 DKIM 配置更新到 DomainConfig 即可。

---