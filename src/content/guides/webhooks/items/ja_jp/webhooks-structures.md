The only structure sent via webhooks is the WebhookComment object, outlined in TypeScript below.

#### WebhookComment オブジェクト構造

##### 「Create」イベント構造  
「create」イベントのリクエストボディは WebhookComment オブジェクトです。

##### 「Update」イベント構造  
「update」イベントのリクエストボディは WebhookComment オブジェクトです。

##### 「Delete」イベント構造  
「delete」イベントのリクエストボディは WebhookComment オブジェクトです。

2023年11月14日以降の変更  
以前は「delete」イベントのリクエストボディはコメント ID のみが含まれていました。現在は削除時点の完全なコメントが含まれます。

すべてのキーは常にボディに存在します。フィールドに値がない場合、ボディは `null`（ブール値の場合は `false`、リストの場合は `[]`）を保持します。そのため、配信の形状はコメントごとに変わりません。

[inline-code-attrs-start title = 'WebhookComment オブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** The id of the comment. **/
    id: string
    /** The id or URL that identifies the comment thread. Normalized. **/
    urlId: string
    /** The URL that points to where the comment was left. **/
    url: string | null
    /** The user id that left the comment. If SSO, prefixed with tenant id. **/
    userId: string | null
    /** The email of the user left the comment. **/
    commenterEmail: string | null
    /** The name of the user that shows in the comment widget. With SSO, can be displayName. **/
    commenterName: string
    /** Raw comment text. **/
    comment: string
    /** Comment text after parsing. **/
    commentHTML: string
    /** Comment external id. **/
    externalId: string | null
    /** The id of the parent comment. **/
    parentId: string | null
    /** The UTC date when the comment was left. **/
    date: UTC_ISO_DateString
    /** Combined karma (up - down) of votes. **/
    votes: number
    votesUp: number
    votesDown: number
    /** True if the user was logged in when they commented, or their verified the comment, or if they verified their session when the comment was left. **/
    verified: boolean
    /** The UTC date when the comment was verified. **/
    verifiedDate: UTC_ISO_DateString | null
    /** If a moderator marked the comment reviewed. **/
    reviewed: boolean
    /** The location, or base64 encoding, of the avatar. Will only be base64 if that was the value passed with SSO. **/
    avatarSrc: string | null
    /** Was the comment manually or automatically marked as spam? **/
    isSpam: boolean
    /** Was the comment automatically marked as spam? **/
    aiDeterminedSpam: boolean
    /** Are there images in the comment? **/
    hasImages: boolean
    /** The page number the comment is on for the "Most Relevant" sort direction. **/
    pageNumber: number | null
    /** The page number the comment is on for the "Oldest First" sort direction. **/
    pageNumberOF: number | null
    /** The page number the comment is on for the "Newest First" sort direction. **/
    pageNumberNF: number | null
    /** Was the comment approved automatically or manually? **/
    approved: boolean
    /** The locale code (format: en_us) of the user when the comment was written. **/
    locale: string | null
    /** The @mentions written in the comment that were successfully parsed. Empty when there are none. **/
    mentions: CommentUserMention[]
    /** The domain the comment is from. **/
    domain: string | null
    /** The moderation group ids associated with this comment. Empty when there are none. **/
    moderationGroupIds: string[]
}
[inline-code-end]

コメント内でユーザーがタグ付けされると、その情報は `mentions` というリストに保存されます。そのリスト内の各オブジェクトは以下の構造を持ちます。

[inline-code-attrs-start title = 'Webhook Mentions オブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** The user id. For SSO users, this will have your tenant id prefixed. **/
    id: string
    /** The final @mention tag text, including the @ symbol. **/
    tag: string
    /** The original @mention tag text, including the @ symbol. **/
    rawTag: string
    /** What type of user was tagged. user = FastComments.com account. sso = SSOUser. **/
    type: 'user'|'sso'
    /** If the user opts out of notifications, this will still be set to true. **/
    sent: boolean
}
[inline-code-end]

#### HTTP メソッド

管理パネルで各 webhook イベントタイプの HTTP メソッドを設定できます：

- **Create Event**: POST または PUT（デフォルト: PUT）
- **Update Event**: POST または PUT（デフォルト: PUT）
- **Delete Event**: DELETE、POST、または PUT（デフォルト: DELETE）

すべてのリクエストに ID が含まれるため、Create と Update の操作はデフォルトで冪等です（PUT）。同じ Create または Update リクエストを繰り返しても、側で重複したオブジェクトは作成されません。

#### リクエストヘッダー

各 webhook リクエストには以下のヘッダーが含まれます：

| Header | Description |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | あなたの API シークレット |
| `X-FastComments-Timestamp` | リクエストが署名されたときの Unix タイムスタンプ（秒） |
| `X-FastComments-Signature` | HMAC-SHA256 署名 (`sha256=<hex>`) |

HMAC 署名の検証に関する情報は、[Security & API Tokens](/guide-webhooks.html#webhooks-api-tokens) を参照してください。