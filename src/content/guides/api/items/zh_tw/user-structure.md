`User` 是一個物件，代表所有使用者的最常見共同基準。

請注意，在 FastComments 中，我們有多種不同的使用者使用情境：

- Secure SSO
- Simple SSO
- 租戶使用者（例如：管理員）
- 留言者

此 API 適用於**留言者**以及透過 **Simple SSO** 建立的使用者。基本上，任何透過您的網站建立的使用者都可以透過此 API 存取。租戶使用者也可以透過此方式取得，但若使用 `/tenant-users/` API，您將能獲得更多資訊。

對於 `Secure SSO`，請使用 `/sso-users/` API。

您無法更新這些類型的使用者。他們是透過您的網站建立帳號的，因此我們僅提供一些基本的唯讀存取權限，但您無法進行更改。如果您想要這種類型的流程，您需要設定 `Secure SSO`。

`User` 物件的結構如下：

[inline-code-attrs-start title = 'User 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** 這也是評論物件中用作 userId 的 id。 **/
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