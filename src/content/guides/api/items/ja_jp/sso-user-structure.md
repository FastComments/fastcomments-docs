FastComments は使いやすい SSO ソリューションを提供します。HMAC ベースの統合でユーザー情報を更新することは、更新されたペイロードでページを読み込ませるだけで簡単に行えます。

しかし、アプリケーションの一貫性を高めるために、そのフローの外でユーザーを管理したい場合もあるでしょう。

SSO User API は SSOUsers と呼ぶオブジェクトを CRUD する方法を提供します。これらのオブジェクトは通常の Users とは異なり、型安全性のために分けて保持されます。

SSOUser オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'SSOUser の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    isAccountOwner?: boolean // 管理者権限 - このフラグを持つ SSO ユーザーは SSO 管理者として課金されます（通常の SSO ユーザーとは別扱い）
    isAdminAdmin?: boolean // 管理者権限 - このフラグを持つ SSO ユーザーは SSO 管理者として課金されます（通常の SSO ユーザーとは別扱い）
    isCommentModeratorAdmin?: boolean // モデレーター権限 - このフラグを持つ SSO ユーザーは SSO モデレーターとして課金されます（通常のモデレーターとは別扱い）
    /** null の場合、アクセスコントロールはユーザーに適用されません。空のリストの場合、このユーザーはどのページも表示できず、他のユーザーを @mention できなくなります。 **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 他のユーザーがこのユーザーのプロフィールでのアクティビティ（コメントを含む）を見られないようにします。既定ではセキュアなプロフィールを提供するために true です。 **/
    isProfileActivityPrivate?: boolean
    /** 他のユーザーがこのユーザーのプロフィールにコメントを残したり、既存のプロフィールコメントを見たりできないようにします。既定は false です。 **/
    isProfileCommentsPrivate?: boolean
    /** 他のユーザーがこのユーザーにダイレクトメッセージを送るのを禁止します。既定は false です。 **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** ユーザーバッジのオプション設定。 **/
    badgeConfig?: {
        /** ユーザーに割り当てるバッジ ID の配列。最大 30 個まで。順序は尊重されます。これらは全ページで表示されるグローバルバッジです。 **/
        badgeIds: string[]
        /** 現在のページ（urlId）にスコープされたバッジ ID の配列。これらのバッジは割り当てられたページでのみ表示されます。 **/
        pageBadgeIds?: string[]
        /** true の場合、既存の表示バッジをすべて提供されたものに置き換えます。グローバルバッジとページスコープバッジは独立して上書きされます。false の場合は既存のバッジに追加されます。 **/
        override?: boolean
        /** true の場合、テナント設定からバッジ表示プロパティを更新します。 **/
        update?: boolean
    }
}
[inline-code-end]

### Billing for SSO Users

SSO ユーザーの課金はその権限フラグに応じて異なります:

- **Regular SSO Users**: 管理者やモデレーター権限を持たないユーザーは通常の SSO ユーザーとして課金されます
- **SSO Admins**: `isAccountOwner` または `isAdminAdmin` フラグを持つユーザーは SSO 管理者として別途課金されます（通常のテナント管理者と同じ料金）
- **SSO Moderators**: `isCommentModeratorAdmin` フラグを持つユーザーは SSO モデレーターとして別途課金されます（通常のモデレーターと同じ料金）

**重要**: 二重課金を防ぐため、システムはメールアドレスで SSO ユーザーを通常のテナントユーザーおよびモデレーターと自動的に重複排除します。SSO ユーザーが通常のテナントユーザーまたはモデレーターと同じメールを持っている場合、二重に課金されることはありません。

### Access Control

ユーザーはグループに分けることができます。これが `groupIds` フィールドの目的であり、オプションです。

### @Mentions

既定では `@` 文字が入力されると `@mentions` は他の sso ユーザーを検索するために `username` を使用します。`displayName` が使用される場合、`displayName` にマッチするものがあると `username` にマッチする結果は無視され、`@mention` の検索結果は `displayName` を使用します。

### Subscriptions

FastComments では、ユーザーはコメントウィジェットのベルアイコンをクリックして Subscribe をクリックすることでページを購読できます。

通常のユーザーの場合、通知設定に基づいて通知メールを送信します。

SSO ユーザーの場合、互換性のためにこれを分けています。ユーザーは `optedInSubscriptionNotifications` を `true` に設定した場合にのみ、これらの追加の購読通知メールが送信されます。

### Badges

`badgeConfig` プロパティを使用して SSO ユーザーにバッジを割り当てることができます。バッジはコメント内のユーザー名の横に表示される視覚的な指標です。

- `badgeIds` - ユーザーに割り当てるバッジ ID の配列。これらはすべてのページで表示されるグローバルバッジです。FastComments アカウントで作成された有効なバッジ ID である必要があります。30 個までに制限されています。
- `pageBadgeIds` - 現在のページ（`urlId`）にスコープされたオプションのバッジ ID の配列。これらのバッジは割り当てられたページでのみ表示されます。同じユーザーに対してページごとに異なるページスコープバッジを設定できます。
- `override` - true の場合、既存の表示バッジをすべて提供されたものに置き換えます。グローバルバッジとページスコープバッジは独立して上書きされます — グローバルバッジの上書きはページスコープバッジに影響を与えませんし、その逆も同様です。false または省略された場合、提供されたバッジは既存のバッジに追加されます。
- `update` - true の場合、ユーザーがログインするたびにテナント設定からバッジ表示プロパティが更新されます。