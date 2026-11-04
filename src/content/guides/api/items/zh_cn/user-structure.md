`User` 是一个对象，代表所有用户的最小公共集合。

请记住，在 FastComments 中我们有许多不同的用户使用场景：

- Secure SSO
- Simple SSO
- 租户用户（例如：管理员）
- 评论者

此 API 适用于**评论者**以及通过 **Simple SSO** 创建的用户。基本上，任何通过您的网站创建的用户都可以通过此 API 访问。租户用户也可以通过这种方式获取，但通过与 `/tenant-users/` API 交互，您将获得更多信息。

对于 `Secure SSO`，请使用 `/sso-users/` API。

您无法更新这些类型的用户。他们是通过您的网站创建的账户，因此我们提供一些基本的只读访问权限，但您无法进行更改。如果您想要这种类型的流程，您需要设置 `Secure SSO`。

`User` 对象的结构如下：

[inline-code-attrs-start title = 'User 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** 这也是评论对象上用作 userId 的 id。 **/
    id: string
    username: string
    /** 例如，指向评论者博客的链接。 **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]