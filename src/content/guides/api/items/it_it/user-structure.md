`User` è un oggetto che rappresenta il denominatore più comune di tutti gli utenti.

Tieni presente che in FastComments abbiamo una serie di casi d'uso diversi per gli utenti:

- Secure SSO
- Simple SSO
- Tenant Users (Ad esempio: Administrators)
- Commenters

This API is for **Commenters** and users created via **Simple SSO**. Basically, any user created
through your site can be accessed via this API. Tenant Users can also be fetched this way, but you'll get more information by interacting with the `/tenant-users/` API.

For `Secure SSO` please use the `/sso-users/` API.

You cannot update these types of users. They created their account through your site, so we provide some basic read-only access, but
you cannot make changes. If you want to have this type of flow - you need to setup `Secure SSO`.

La struttura dell'oggetto `User` è la seguente:

[inline-code-attrs-start title = 'Struttura di User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** Questo è anche l'id usato come userId sugli oggetti commento. **/
    id: string
    username: string
    /** Un link, ad esempio al blog del commentatore. **/
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