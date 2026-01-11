唯一ウェブフック経由で送信される構造は、以下の TypeScript で示された WebhookComment オブジェクトです。

#### WebhookComment オブジェクトの構造

##### "Create" イベントの構造
"create" イベントのリクエストボディは WebhookComment オブジェクトです。

##### "Update" イベントの構造
"update" イベントのリクエストボディは WebhookComment オブジェクトです。

##### "Delete" イベントの構造
"delete" イベントのリクエストボディは WebhookComment オブジェクトです。

    2023年11月14日の変更
    以前は "delete" イベントのリクエストボディにはコメントの id のみが含まれていました。現在は削除時のフルコメントが含まれます。


[inline-code-attrs-start title = 'WebhookComment オブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** コメントの id **/
    id: string
    /** コメントスレッドを識別する id または URL。正規化済み。 **/
    urlId: string
    /** コメントが残された場所を指す URL。 **/
    url?: string
    /** コメントを残したユーザーの id。SSO の場合はテナント id がプレフィックスとして付与されます。 **/
    userId?: string
    /** コメントを残したユーザーのメールアドレス。 **/
    commenterEmail?: string
    /** コメントウィジェットに表示されるユーザー名。SSO の場合は displayName であることがあります。 **/
    commenterName: string
    /** 生のコメントテキスト。 **/
    comment: string
    /** 解析後のコメントテキスト。 **/
    commentHTML: string
    /** コメントの外部 id。 **/
    externalId?: string
    /** 親コメントの id。 **/
    parentId?: string | null
    /** コメントが残された UTC 日時。 **/
    date: UTC_ISO_DateString
    /** 投票の合計カルマ（賛成 - 反対）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** ユーザーがコメント時にログインしていた、またはコメントが検証されていた、あるいはコメント時にセッションが検証されていた場合は true。 **/
    verified: boolean
    /** コメントが検証された日時。 **/
    verifiedDate?: number
    /** モデレーターがコメントを確認済みにしたかどうか。 **/
    reviewed: boolean
    /** アバターの場所、または base64 エンコード。SSO で渡された値が base64 の場合のみ base64 になります。 **/
    avatarSrc?: string
    /** コメントが手動または自動でスパムとマークされたか？ **/
    isSpam: boolean
    /** コメントが自動的にスパムと判定されたか？ **/
    aiDeterminedSpam: boolean
    /** コメントに画像が含まれているか？ **/
    hasImages: boolean
    /** 「最も関連性の高い」ソート方向でのコメントのページ番号。 **/
    pageNumber: number
    /** 「古い順」ソート方向でのコメントのページ番号。 **/
    pageNumberOF: number
    /** 「新しい順」ソート方向でのコメントのページ番号。 **/
    pageNumberNF: number
    /** コメントが自動的または手動で承認されたか？ **/
    approved: boolean
    /** コメント作成時のユーザーのロケールコード（形式: en_us）。 **/
    locale: string
    /** コメント内で書かれ、正常に解析された @mentions。 **/
    mentions?: CommentUserMention[]
    /** コメントの元のドメイン。 **/
    domain?: string
    /** このコメントに関連付けられたモデレーショングループの id のオプションリスト。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

ユーザーがコメント内でタグ付けされた場合、その情報は `mentions` というリストに格納されます。そのリスト内の各オブジェクトは以下の構造を持ちます。

[inline-code-attrs-start title = 'Webhook メンションオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ユーザー id。SSO ユーザーの場合、テナント id がプレフィックスとして付与されます。 **/
    id: string
    /** 最終的な @mention タグのテキスト（@ 記号を含む）。 **/
    tag: string
    /** 元の @mention タグのテキスト（@ 記号を含む）。 **/
    rawTag: string
    /** タグ付けされたユーザーの種類。user = FastComments.com アカウント、sso = SSOUser。 **/
    type: 'user'|'sso'
    /** ユーザーが通知をオプトアウトしている場合でも、これは true に設定されます。 **/
    sent: boolean
}
[inline-code-end]

#### HTTP メソッド

管理パネルで各ウェブフックイベントタイプの HTTP メソッドを設定できます:

- **Create Event**: POST または PUT (デフォルト: PUT)
- **Update Event**: POST または PUT (デフォルト: PUT)
- **Delete Event**: DELETE、POST、または PUT (デフォルト: DELETE)

すべてのリクエストが ID を含むため、Create および Update 操作はデフォルトで冪等（PUT）です。同じ Create または Update リクエストを繰り返しても、あなたの側でオブジェクトが重複して作成されることはないはずです。

#### リクエストヘッダー

各ウェブフックリクエストには次のヘッダーが含まれます:

| ヘッダー | 説明 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | あなたの API シークレット |
| `X-FastComments-Timestamp` | リクエストが署名された Unix タイムスタンプ（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 署名（`sha256=<hex>`） |

詳細な HMAC 署名の検証方法は [セキュリティと API トークン](/guides/webhooks/webhooks-api-tokens) を参照してください。