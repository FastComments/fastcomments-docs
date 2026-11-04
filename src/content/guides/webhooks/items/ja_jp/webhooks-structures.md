Webhook経由で送信される構造はWebhookCommentオブジェクトのみで、以下にTypeScriptで示します。

#### WebhookCommentオブジェクトの構造

##### "Create"イベントの構造
"create"イベントのリクエストボディはWebhookCommentオブジェクトです。

##### "Update"イベントの構造
"update"イベントのリクエストボディはWebhookCommentオブジェクトです。

##### "Delete"イベントの構造
"delete"イベントのリクエストボディはWebhookCommentオブジェクトです。

    2023年11月14日時点の変更
    以前は"delete"イベントのリクエストボディにはコメントIDのみが含まれていました。現在は削除時点のコメント全体が含まれます。


すべてのキーは常にボディに存在します。コメントにフィールドの値がない場合、ボディには `null`(ブール値の場合は `false`、リストの場合は `[]`)が格納されるため、配信の形式はコメントごとに変わりません。

[inline-code-attrs-start title = 'WebhookCommentオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** コメントのID。 **/
    id: string
    /** コメントスレッドを識別するIDまたはURL。正規化されています。 **/
    urlId: string
    /** コメントが投稿された場所を指すURL。 **/
    url: string | null
    /** コメントを投稿したユーザーのID。SSOの場合はテナントIDが接頭辞として付きます。 **/
    userId: string | null
    /** コメントを投稿したユーザーのメールアドレス。 **/
    commenterEmail: string | null
    /** コメントウィジェットに表示されるユーザーの名前。SSOの場合はdisplayNameになることがあります。 **/
    commenterName: string
    /** 生のコメントテキスト。 **/
    comment: string
    /** パース後のコメントテキスト。 **/
    commentHTML: string
    /** コメントの外部ID。 **/
    externalId: string | null
    /** 親コメントのID。 **/
    parentId: string | null
    /** コメントが投稿されたUTC日時。 **/
    date: UTC_ISO_DateString
    /** 投票の合計カルマ(賛成 - 反対)。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** ユーザーがコメント時にログインしていたか、コメントを認証したか、コメント投稿時にセッションを認証した場合にtrue。 **/
    verified: boolean
    /** コメントが認証されたUTC日時。 **/
    verifiedDate: UTC_ISO_DateString | null
    /** モデレーターがコメントをレビュー済みとしてマークしたかどうか。 **/
    reviewed: boolean
    /** アバターの場所、またはbase64エンコーディング。base64になるのはSSOでその値が渡された場合のみです。 **/
    avatarSrc: string | null
    /** コメントが手動または自動でスパムとしてマークされたかどうか。 **/
    isSpam: boolean
    /** コメントが自動的にスパムとしてマークされたかどうか。 **/
    aiDeterminedSpam: boolean
    /** コメントに画像が含まれているかどうか。 **/
    hasImages: boolean
    /** "Most Relevant"ソート方向でのコメントのページ番号。 **/
    pageNumber: number | null
    /** "Oldest First"ソート方向でのコメントのページ番号。 **/
    pageNumberOF: number | null
    /** "Newest First"ソート方向でのコメントのページ番号。 **/
    pageNumberNF: number | null
    /** コメントが自動または手動で承認されたかどうか。 **/
    approved: boolean
    /** コメント作成時のユーザーのロケールコード(形式: en_us)。 **/
    locale: string | null
    /** コメント内で正常にパースされた@メンション。ない場合は空。 **/
    mentions: CommentUserMention[]
    /** コメントの送信元ドメイン。 **/
    domain: string | null
    /** このコメントに関連付けられたモデレーショングループのID。ない場合は空。 **/
    moderationGroupIds: string[]
    /** このコメントでコメント投稿者の名前の横に表示されるバッジ(イベント作成時点のもの)。ない場合は空。 **/
    badges: CommentUserBadgeInfo[]
}
[inline-code-end]

コメント内でユーザーがタグ付けされると、その情報は `mentions` というリストに保存されます。そのリスト内の各オブジェクトは以下の構造を持ちます。

[inline-code-attrs-start title = 'Webhookメンションオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ユーザーID。SSOユーザーの場合はテナントIDが接頭辞として付きます。 **/
    id: string
    /** @記号を含む最終的な@メンションタグのテキスト。 **/
    tag: string
    /** @記号を含む元の@メンションタグのテキスト。 **/
    rawTag: string
    /** タグ付けされたユーザーの種類。user = FastComments.comアカウント。sso = SSOUser。 **/
    type: 'user'|'sso'
    /** ユーザーが通知をオプトアウトしていても、これはtrueに設定されたままになります。 **/
    sent: boolean
}
[inline-code-end]

`badges` リストの各エントリは以下の構造を持ちます。バッジの種類については[バッジガイド](/guide-badges.html)を参照してください。

[inline-code-attrs-start title = 'Webhookバッジオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserBadgeInfo {
    /** このユーザーに付与されたバッジのID。 **/
    id: string
    /** バッジの種類。 **/
    type: number
    /** バッジの説明。バッジにホバーすると表示されます。 **/
    description: string
    /** バッジに表示されるテキスト。 **/
    displayLabel?: string | null
    /** バッジに表示される画像(使用する場合)。 **/
    displaySrc?: string | null
    backgroundColor?: string | null
    borderColor?: string | null
    textColor?: string | null
    /** バッジに適用される追加のCSSクラス。 **/
    cssClass?: string | null
}
[inline-code-end]

#### HTTPメソッド

管理パネルで各WebhookイベントタイプのHTTPメソッドを設定できます:

- **Createイベント**: POSTまたはPUT(デフォルト: PUT)
- **Updateイベント**: POSTまたはPUT(デフォルト: PUT)
- **Deleteイベント**: DELETE、POST、またはPUT(デフォルト: DELETE)

すべてのリクエストにIDが含まれているため、CreateおよびUpdate操作はデフォルト(PUT)で冪等です。同じCreateまたはUpdateリクエストを繰り返しても、こちら側で重複したオブジェクトが作成されることはありません。

#### リクエストヘッダー

各Webhookリクエストには以下のヘッダーが含まれます:

| ヘッダー | 説明 |
|--------|-------------|
| `Content-Type` | `application/json` |
| `token` | あなたのAPIシークレット |
| `X-FastComments-Timestamp` | リクエストが署名された時点のUnixタイムスタンプ(秒) |
| `X-FastComments-Signature` | HMAC-SHA256署名(`sha256=<hex>`) |

HMAC署名の検証方法については、[セキュリティとAPIトークン](/guide-webhooks.html#webhooks-api-tokens)を参照してください。