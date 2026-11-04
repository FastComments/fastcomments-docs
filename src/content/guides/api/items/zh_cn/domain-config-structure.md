`DomainConfig` 对象表示租户某个域名的配置。

`DomainConfig` 对象的结构如下：

[inline-code-attrs-start title = '域名配置结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfig {
    /** 一个域名，不是 URL，例如 "fastcomments.com" 或 "www.example.com"。如果希望限制到子域名，可以包含子域名。最多 1000 个字符。 **/
    domain: string
    /** 发送邮件时使用的发件人名称。 **/
    emailFromName?: string
    /** 发送邮件时使用的发件人邮箱。请确保已设置 SPF，允许 mail.fastcomments.com 以此属性中使用的域名发送邮件。 **/
    emailFromEmail?: string
    /** 只读。对象创建的时间。 **/
    createdAt: string
    /** 与此域名相关的 logo。用于邮件中。请使用 HTTPS。 **/
    logoSrc?: string
    /** 与此域名相关的较小 logo。请使用 HTTPS。 **/
    logoSrc100px?: string
    /** 仅限 SSO。在发送的每封邮件页脚中使用的 URL。支持 "[userId]" 变量。 **/
    footerUnsubscribeURL?: string
    /** 仅限 SSO。在发送的每封邮件中使用的邮件头。例如可用于设置与退订相关的邮件头以提高送达率。此 Record 中的 List-Unsubscribe 条目（如果存在）支持 "[userId]" 变量。 **/
    emailHeaders?: Record<string, string>
    /** 禁用所有退订链接。不推荐，可能会影响送达率。 **/
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
    /** @deprecated 不再在 API 响应中返回。为向后兼容，写入时仍可接受。 **/
    privateKey?: string
}
[inline-code-end]

### 用于身份验证

域名配置用于确定哪些网站可以为您的账户托管 FastComments 组件。这是一种基本的身份验证形式，这意味着添加或删除任何域名配置都可能影响您的 FastComments 安装在生产环境中的可用性。

除非有意禁用该域名，否则不要删除或更新当前正在使用的域名的 `Domain Config` 的 `domain` 属性。

这与在 [/auth/my-account/configure-domains](https://fastcomments.com/auth/my-account/configure-domains) 中移除域名的行为相同。

另请注意，从 `My Domains` 界面中移除域名时，也会删除通过该界面为该域名添加的所有相应配置。

### 用于邮件自定义

邮件页脚中的退订链接，以及许多邮件客户端提供的一键退订功能，可以通过此 API 分别通过定义 `footerUnsubscribeURL` 和 `emailHeaders` 进行配置。

### 用于 DKIM

定义好 DKIM DNS 记录后，只需使用定义的结构更新包含 DKIM 配置的 DomainConfig 即可。