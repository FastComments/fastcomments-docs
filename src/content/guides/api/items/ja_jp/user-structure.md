`User` はすべてのユーザーの最も一般的な共通要素を表すオブジェクトです。

Keep in mind that at FastComments we have a bunch of different use cases for users:

- Secure SSO
- Simple SSO
- Tenant Users (For example: Administrators)
- Commenters

This API is for **Commenters** and users created via **Simple SSO**. Basically, any user created
through your site can be accessed via this API. Tenant Users can also be fetched this way, but you'll get more information by interacting with the `/tenant-users/` API.

For `Secure SSO` please use the `/sso-users/` API.

You cannot update these types of users. They created their account through your site, so we provide some basic read-only access, but
you cannot make changes. If you want to have this type of flow - you need to setup `Secure SSO`.

The structure for the `User` object is as follows:

[inline-code-attrs-start title = 'User の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** これはコメントオブジェクトの userId としても使用される id です。 **/
    id: string
    username: string
    /** 例えば、コメント投稿者のブログへのリンク。 **/
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