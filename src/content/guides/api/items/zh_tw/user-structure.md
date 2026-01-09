`User` is an object that represents a most-common denominator of all users.

Keep in mind that at FastComments we have a bunch of different use cases for users:

- 安全 SSO
- 簡易 SSO
- 租戶使用者（例如：管理者）
- 留言者

This API is for **留言者** and users created via **簡易 SSO**. Basically, any user created
through your site can be accessed via this API. Tenant Users can also be fetched this way, but you'll get more information by interacting with the `/tenant-users/` API.

For `Secure SSO` please use the `/sso-users/` API.

You cannot update these types of users. They created their account through your site, so we provide some basic read-only access, but
you cannot make changes. If you want to have this type of flow - you need to setup `Secure SSO`.

The structure for the `User` object is as follows:

[inline-code-attrs-start title = '使用者結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** 這也是在評論物件上用作 userId 的 id。 **/
    id: string
    username: string
    /** 例如：留言者部落格的連結。 **/
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