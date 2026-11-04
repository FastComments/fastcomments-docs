FastCommentsは、使いやすいSSOソリューションを提供しています。HMACベースの統合でユーザー情報を更新するのは、更新されたペイロードと共にユーザーにページを読み込ませるだけと簡単です。

しかし、アプリケーションの一貫性を向上させるために、そのフロー外でユーザーを管理したい場合もあるでしょう。

SSO User APIは、SSOUsersと呼ぶオブジェクトをCRUD操作する方法を提供します。これらのオブジェクトは通常のUsersとは異なり、型安全性のために分けて管理されています。

SSOUserオブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'SSOUser 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUser {
    id: string
    username: string
    email?: string
    websiteUrl?: string
    signUpDate: number
    createdFromUrlId?: string
    loginCount?: number
    avatarSrc?: string
    optedInNotifications?: boolean
    optedInSubscriptionNotifications?: boolean
    displayLabel?: string
    displayName?: string
    isAccountOwner?: boolean // 管理者権限 - このフラグを持つSSOユーザーはSSO Adminsとして課金されます(通常のSSOユーザーとは別)
    isAdminAdmin?: boolean // 管理者権限 - このフラグを持つSSOユーザーはSSO Adminsとして課金されます(通常のSSOユーザーとは別)
    isCommentModeratorAdmin?: boolean // モデレーター権限 - このフラグを持つSSOユーザーはSSO Moderatorsとして課金されます(通常のSSOユーザーとは別)
    /** nullの場合、Access Controlはユーザーに適用されません。空のリストの場合、このユーザーはどのページも表示できず、他のユーザーを@mentionすることもできません。 **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 他のユーザーが、コメントを含むこのユーザーのプロフィール上のアクティビティを見られないようにします。デフォルトはtrueで、デフォルトで安全なプロフィールを提供します。 **/
    isProfileActivityPrivate?: boolean
    /** 他のユーザーがユーザーのプロフィールにコメントを残したり、既存のプロフィールコメントを見たりできないようにします。デフォルトはfalseです。 **/
    isProfileCommentsPrivate?: boolean
    /** 他のユーザーがこのユーザーにダイレクトメッセージを送れないようにします。デフォルトはfalseです。 **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** ユーザーバッジのオプション設定。 **/
    badgeConfig?: {
        /** ユーザーに割り当てるバッジIDの配列。最大30個のバッジ。順序は維持されます。これらはすべてのページで表示されるグローバルバッジです。 **/
        badgeIds: string[]
        /** 現在のページ(urlId)にスコープされたバッジIDの配列。これらのバッジは割り当てられたページでのみ表示されます。 **/
        pageBadgeIds?: string[]
        /** trueの場合、既存の表示中のバッジをすべて指定されたバッジに置き換えます。グローバルバッジとページスコープのバッジはそれぞれ独立して上書きされます。falseの場合、既存のバッジに追加されます。 **/
        override?: boolean
        /** trueの場合、テナント設定からバッジの表示プロパティを更新します。 **/
        update?: boolean
    }
}
[inline-code-end]

### SSOユーザーの課金

SSOユーザーは、権限フラグに応じて異なる課金が行われます:

- **通常のSSOユーザー**: 管理者またはモデレーター権限を持たないユーザーは、通常のSSOユーザーとして課金されます
- **SSO Admins**: `isAccountOwner` または `isAdminAdmin` フラグを持つユーザーは、SSO Adminsとして別途課金されます(通常のテナント管理者と同じ料金)
- **SSO Moderators**: `isCommentModeratorAdmin` フラグを持つユーザーは、SSO Moderatorsとして別途課金されます(通常のモデレーターと同じ料金)

**重要**: 二重課金を防ぐため、システムはメールアドレスによってSSOユーザーを通常のテナントユーザーやモデレーターと自動的に重複排除します。SSOユーザーが通常のテナントユーザーやモデレーターと同じメールアドレスを持つ場合、二重に課金されることはありません。

### Access Control

ユーザーはグループに分けることができます。これが `groupIds` フィールドの目的であり、オプションです。

### @Mentions

デフォルトでは、`@` 文字が入力されると `@mentions` は `username` を使用して他のSSOユーザーを検索します。`displayName` が使用されている場合、`displayName` に一致する結果があるときは `username` に一致する結果は無視され、`@mention` の検索結果には `displayName` が使用されます。

### Subscriptions

FastCommentsでは、ユーザーはコメントウィジェットのベルのアイコンをクリックしてSubscribeをクリックすることで、ページを購読できます。

通常のユーザーの場合、通知設定に基づいて通知メールを送信します。

SSOユーザーの場合、後方互換性のためにこれを分けています。`optedInSubscriptionNotifications` を `true` に設定した場合にのみ、これらの追加の購読通知メールが送信されます。

### Badges

`badgeConfig` プロパティを使用して、SSOユーザーにバッジを割り当てることができます。バッジは、コメント内のユーザー名の横に表示される視覚的なインジケーターです。

- `badgeIds` - ユーザーに割り当てるバッジIDの配列。これらはすべてのページで表示されるグローバルバッジです。FastCommentsアカウントで作成された有効なバッジIDである必要があります。最大30個のバッジまでです。
- `pageBadgeIds` - 現在のページ(`urlId`)にスコープされたバッジIDのオプションの配列。これらのバッジは割り当てられたページでのみ表示されます。同じユーザーに対して、異なるページに異なるページスコープのバッジを設定できます。
- `override` - trueの場合、既存の表示中のバッジはすべて指定されたバッジに置き換えられます。グローバルバッジとページスコープのバッジはそれぞれ独立して上書きされます — グローバルバッジの上書きはページスコープのバッジに影響せず、その逆も同様です。falseまたは省略した場合、指定されたバッジは既存のバッジに追加されます。
- `update` - trueの場合、ユーザーがログインするたびに、テナント設定からバッジの表示プロパティが更新されます。