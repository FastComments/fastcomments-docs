`User` 是表示所有用户的最通用属性的对象。

请记住，在 FastComments 中，我们有多种不同的用户使用场景：

- Secure SSO
- Simple SSO
- Tenant Users (例如：管理员)
- Commenters

This API is for **Commenters** and users created via **Simple SSO**. Basically, any user created
through your site can be accessed via this API. Tenant Users can also be fetched this way, but you'll get more information by interacting with the `/tenant-users/` API.

For `Secure SSO` please use the `/sso-users/` API.

You cannot update these types of users. They created their account through your site, so we provide some basic read-only access, but
you cannot make changes. If you want to have this type of flow - you need to setup `Secure SSO`.

The structure for the `User` object is as follows:

[inline-code-attrs-start title = '用户结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** 这也是在评论对象上作为 userId 使用的 id。 **/
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

---