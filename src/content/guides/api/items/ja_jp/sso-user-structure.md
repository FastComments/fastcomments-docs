FastComments は使いやすい SSO ソリューションを提供します。HMAC ベースの統合を使用したユーザー情報の更新は、ユーザーが更新されたペイロードでページを読み込むだけで簡単に行えます。

ただし、アプリケーションの整合性を高めるために、そのフロー外でユーザーを管理したい場合もあります。

SSO User API は、SSOUsers と呼ぶオブジェクトを CRUD する方法を提供します。これらのオブジェクトは通常の Users と異なり、型安全性のために別に保持されています。

SSOUser オブジェクトの構造は次のとおりです：

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
    isAccountOwner?: boolean // 管理者権限 - このフラグが付いた SSO ユーザーは SSO 管理者として課金されます（通常の SSO ユーザーとは別扱い）
    isAdminAdmin?: boolean // 管理者権限 - このフラグが付いた SSO ユーザーは SSO 管理者として課金されます（通常の SSO ユーザーとは別扱い）
    isCommentModeratorAdmin?: boolean // モデレーター権限 - このフラグが付いた SSO ユーザーは SSO モデレーターとして課金されます（通常の SSO ユーザーとは別扱い）
    /** null の場合、アクセス制御はユーザーに適用されません。空のリストの場合、このユーザーはページを閲覧したり他のユーザーを @mention することができなくなります。 **/
    groupIds?: string[] | null
    createdFromSimpleSSO?: boolean
    /** 他のユーザーがこのユーザーのプロフィール上のアクティビティ（コメントを含む）を見られないようにします。デフォルトは安全なプロフィールを提供するため true です。 **/
    isProfileActivityPrivate?: boolean
    /** 他のユーザーがこのユーザーのプロフィールにコメントを残したり、既存のプロフィールコメントを見たりできないようにします。デフォルトは false です。 **/
    isProfileCommentsPrivate?: boolean
    /** 他のユーザーがこのユーザーにダイレクトメッセージを送信できないようにします。デフォルトは false です。 **/
    isProfileDMDisabled?: boolean
    karma?: number
    /** ユーザーバッジのオプション設定。 **/
    badgeConfig?: {
        /** ユーザーに割り当てるバッジ ID の配列。30 個までに制限されています。順序は尊重されます。 **/
        badgeIds: string[]
        /** true の場合、表示されている既存のバッジをすべて提供されたバッジで置き換えます。false の場合、既存のバッジに追加します。 **/
        override?: boolean
        /** true の場合、テナント設定からバッジの表示プロパティを更新します。 **/
        update?: boolean
    }
}
[inline-code-end]

### SSO ユーザーの課金

SSO ユーザーは権限フラグに応じて異なる方法で課金されます：

- **通常の SSO ユーザー**: 管理者またはモデレーター権限を持たないユーザーは通常の SSO ユーザーとして課金されます
- **SSO 管理者**: `isAccountOwner` または `isAdminAdmin` フラグを持つユーザーは、別個に SSO 管理者として課金されます（通常のテナント管理者と同じ料金）
- **SSO モデレーター**: `isCommentModeratorAdmin` フラグを持つユーザーは、別個に SSO モデレーターとして課金されます（通常のモデレーターと同じ料金）

**重要**: 二重課金を防ぐため、システムはメールアドレスで SSO ユーザーを通常のテナントユーザーおよびモデレーターと自動的に重複排除します。SSO ユーザーが通常のテナントユーザーまたはモデレーターと同じメールを持っている場合、二重に課金されることはありません。

### アクセス制御

ユーザーはグループに分けることができます。これが `groupIds` フィールドの目的で、オプションです。

### @メンション

デフォルトでは、`@` を入力すると `@mentions` は `username` を使って他の SSO ユーザーを検索します。`displayName` が使用されている場合、`displayName` に一致する結果があるときは `username` に一致する結果は無視され、`@mention` の検索結果は `displayName` を使用します。

### サブスクリプション

FastComments では、ユーザーはコメントウィジェットのベルアイコンをクリックして「購読」をクリックすることでページを購読できます。

通常ユーザーの場合、通知設定に基づいて通知メールを送信します。

SSO ユーザーについては、後方互換性のためにこれを区別しています。追加の購読通知メールは `optedInSubscriptionNotifications` を `true` に設定した場合にのみ送信されます。

### バッジ

`badgeConfig` プロパティを使って SSO ユーザーにバッジを割り当てることができます。バッジはコメント内でユーザー名の横に表示される視覚的な表示です。

- `badgeIds` - ユーザーに割り当てるバッジ ID の配列。これらはあなたの FastComments アカウントで作成された有効なバッジ ID である必要があります。30 個までに制限されています。
- `override` - true の場合、コメントに表示されている既存のバッジはすべて提供されたバッジで置き換えられます。false または省略した場合、提供されたバッジは既存のバッジに追加されます。
- `update` - true の場合、ユーザーがログインするたびにテナントの設定からバッジの表示プロパティが更新されます。