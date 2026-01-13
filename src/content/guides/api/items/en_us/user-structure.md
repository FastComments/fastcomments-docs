`User` is an object that represents a most-common denominator of all users.

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

[inline-code-attrs-start title = 'User Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
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
