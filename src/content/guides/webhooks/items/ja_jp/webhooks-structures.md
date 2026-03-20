Webhookを介して送信される構造は、下記にTypeScriptで示した WebhookComment オブジェクトのみです。

#### WebhookComment オブジェクトの構造

##### "create" イベントの構造
"create" イベントのリクエストボディは WebhookComment オブジェクトです。

##### "update" イベントの構造
"update" イベントのリクエストボディは WebhookComment オブジェクトです。

##### "delete" イベントの構造
"delete" イベントのリクエストボディは WebhookComment オブジェクトです。

    2023年11月14日の変更
    以前は "delete" イベントのリクエストボディはコメントIDのみを含んでいました。新しくなり、削除時の完全なコメントを含むようになりました。


[inline-code-attrs-start title = 'WebhookComment オブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** コメントのID。 **/
    id: string
    /** コメントスレッドを識別するIDまたはURL。正規化済み。 **/
    urlId: string
    /** コメントが投稿された場所を指すURL。 **/
    url?: string
    /** コメントを投稿したユーザーのID。SSOの場合、テナントIDがプレフィックスされる。 **/
    userId?: string
    /** コメントを残したユーザーのメール。 **/
    commenterEmail?: string
    /** コメントウィジェットに表示されるユーザー名。SSOではdisplayNameになることがある。 **/
    commenterName: string
    /** 未加工のコメントテキスト。 **/
    comment: string
    /** パース後のコメントテキスト。 **/
    commentHTML: string
    /** 外部コメントID。 **/
    externalId?: string
    /** 親コメントのID。 **/
    parentId?: string | null
    /** コメントが残されたUTC日時。 **/
    date: UTC_ISO_DateString
    /** 投票の合計カルマ（賛成 - 反対）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** コメント投稿時にユーザーがログインしていた、またはコメントが認証済みである、あるいはセッションが検証されていた場合はtrue。 **/
    verified: boolean
    /** コメントが検証された日時。 **/
    verifiedDate?: number
    /** モデレーターがコメントを確認済みにしたかどうか。 **/
    reviewed: boolean
    /** アバターの場所、またはbase64エンコード。SSOで渡された値がbase64だった場合のみbase64になる。 **/
    avatarSrc?: string
    /** コメントが手動または自動でスパムとしてマークされたかどうか。 **/
    isSpam: boolean
    /** コメントが自動的にスパムとマークされたかどうか。 **/
    aiDeterminedSpam: boolean
    /** コメント内に画像があるかどうか。 **/
    hasImages: boolean
    /** 「Most Relevant」ソート順でのコメントのページ番号。 **/
    pageNumber: number
    /** 「Oldest First」ソート順でのコメントのページ番号。 **/
    pageNumberOF: number
    /** 「Newest First」ソート順でのコメントのページ番号。 **/
    pageNumberNF: number
    /** コメントが自動的にまたは手動で承認されたかどうか。 **/
    approved: boolean
    /** コメントが書かれたときのユーザーのロケールコード（形式: en_us）。 **/
    locale: string
    /** コメント内で書かれ、正常にパースされた@mentions。 **/
    mentions?: CommentUserMention[]
    /** コメントが投稿されたドメイン。 **/
    domain?: string
    /** このコメントに関連付けられたモデレーショングループIDの任意のリスト。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

ユーザーがコメント内でタグ付けされると、その情報は `mentions` というリストに保存されます。そのリスト内の各オブジェクトは次の構造を持ちます。

[inline-code-attrs-start title = 'Webhook メンションオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ユーザーID。SSOユーザーの場合、テナントIDがプレフィックスされる。 **/
    id: string
    /** 最終的な@mentionタグのテキスト（@記号を含む）。 **/
    tag: string
    /** 元の@mentionタグのテキスト（@記号を含む）。 **/
    rawTag: string
    /** タグ付けされたユーザーのタイプ。user = FastComments.comアカウント、sso = SSOUser。 **/
    type: 'user'|'sso'
    /** ユーザーが通知をオプトアウトしても、これはtrueのままになる。 **/
    sent: boolean
}
[inline-code-end]

#### HTTP メソッド

管理パネルで各WebhookイベントタイプのHTTPメソッドを設定できます：

- **Create Event**: POST or PUT (default: PUT)
- **Update Event**: POST or PUT (default: PUT)
- **Delete Event**: DELETE, POST, or PUT (default: DELETE)

すべてのリクエストがIDを含むため、CreateおよびUpdate操作はデフォルトで冪等（PUT）です。同じCreateまたはUpdateリクエストを繰り返しても、あなたの側で重複したオブジェクトが作成されるべきではありません。

#### リクエストヘッダー

各Webhookリクエストには以下のヘッダーが含まれます：

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | あなたのAPIシークレット |
| `X-FastComments-Timestamp` | リクエストが署名された時刻のUnixタイムスタンプ（秒） |
| `X-FastComments-Signature` | HMAC-SHA256署名 (`sha256=<hex>`) |

HMAC署名の検証に関する情報は [セキュリティとAPIトークン](/guide-webhooks.html#webhooks-api-tokens) を参照してください。