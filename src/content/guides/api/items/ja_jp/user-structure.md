`User` は、すべてのユーザーの共通項を表すオブジェクトです。

FastComments では、ユーザーに複数のユースケースがあることに注意してください:

- Secure SSO
- Simple SSO
- テナントユーザー(例: 管理者)
- コメント投稿者

この API は**コメント投稿者**と **Simple SSO** で作成されたユーザー向けです。基本的に、あなたのサイトを通じて作成されたすべてのユーザーはこの API でアクセスできます。テナントユーザーもこの方法で取得できますが、`/tenant-users/` API を使用した方が多くの情報を得られます。

`Secure SSO` の場合は、`/sso-users/` API をご利用ください。

これらのタイプのユーザーは更新できません。彼らはあなたのサイトを通じてアカウントを作成したため、基本的な読み取り専用アクセスのみを提供していますが、変更を加えることはできません。このようなフローを実現したい場合は、`Secure SSO` を設定する必要があります。

`User` オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'User 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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