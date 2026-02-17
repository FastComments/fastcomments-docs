一个 `DomainConfig` 对象表示针对租户的域配置。

The structure for the `DomainConfig` object is as follows:

[inline-code-attrs-start title = '域配置结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 一个域名，而不是 URL，例如 "fastcomments.com" 或 "www.example.com"。如果希望限制到子域，可以包含子域。最大1000个字符。 **/
    domain: string
    /** 发送邮件时使用的发件人名称。 **/
    emailFromName?: string
    /** 发送邮件时使用的发件人邮箱。请确保已设置 SPF，以允许 mail.fastcomments.com 代表此属性中使用的域发送邮件。 **/
    emailFromEmail?: string
    /** READONLY. 对象创建时间。 **/
    createdAt: string
    /** 与此域相关的徽标。用于邮件。请使用 HTTPS。 **/
    logoSrc?: string
    /** 与此域相关的小尺寸徽标。请使用 HTTPS。 **/
    logoSrc100px?: string
    /** SSO ONLY. 用于每封发送邮件底部的 URL。支持 "[userId]" 变量。 **/
    footerUnsubscribeURL?: string
    /** SSO ONLY. 用于每封发送邮件的头部。例如可用于设置与退订相关的头部以提高投递率。此 Record 中的 List-Unsubscribe 条目（如果存在）支持 "[userId]" 变量。 **/
    emailHeaders?: Record<string, string>
    /** 禁用所有退订链接。不推荐，可能会影响投递率。 **/
    disableUnsubscribeLinks?: boolean
    /** DKIM 配置。 **/
    dkim?: DomainConfigDKIM
}
[inline-code-end]

[inline-code-attrs-start title = 'DKIM 配置结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigDKIM {
    /** DKIM 记录中的域名。 **/
    domainName: string
    /** 要使用的 DKIM 密钥选择器。 **/
    keySelector: string
    /** 公钥，PEM 格式。在 GET 响应中返回。 **/
    publicKey: string
    /** @deprecated 不再在 API 响应中返回。为向后兼容在写入时仍然接受。 **/
    privateKey?: string
}
[inline-code-end]

### 用于身份验证

域配置用于确定哪些站点可以为您的账户承载 FastComments 小部件。这是一种基本形式的认证，这意味着添加或删除任何域配置都可能影响您在生产环境中 FastComments 安装的可用性。

不要删除或更新当前正在使用的域的 `Domain Config` 的 `domain` 属性，除非您有意停用该域。

This has the same behavior as removing a domain from [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains).

另外请注意，从 `My Domains` 界面移除域将删除通过该界面添加的任何对应域配置。

### 用于电子邮件自定义

电子邮件底部的退订链接以及许多邮件客户端提供的一键退订功能，可以通过此 API 分别通过定义 `footerUnsubscribeURL` 和 `emailHeaders` 进行配置。

### 用于 DKIM

在定义 DKIM DNS 记录之后，只需使用上述结构将您的 DKIM 配置更新到 DomainConfig 即可。