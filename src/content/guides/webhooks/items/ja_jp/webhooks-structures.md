Webhook で送信される唯一の構造は WebhookComment オブジェクトで、以下に TypeScript で示します。

#### WebhookComment オブジェクトの構造

##### 「create」イベントの構造
「create」イベントのリクエストボディは WebhookComment オブジェクトです。

##### 「update」イベントの構造
「update」イベントのリクエストボディは WebhookComment オブジェクトです。

##### 「delete」イベントの構造
「delete」イベントのリクエストボディは WebhookComment オブジェクトです。

    2023年11月14日時点での変更
    以前は「delete」イベントのリクエストボディにはコメントの id のみが含まれていました。現在は削除時点の完全なコメントが含まれます。


[inline-code-attrs-start title = 'WebhookComment オブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface WebhookComment {
    /** コメントの id。 **/
    id: string
    /** コメントスレッドを識別する id または URL。正規化済み。 **/
    urlId: string
    /** コメントが残された場所を指す URL。 **/
    url?: string
    /** コメントを残したユーザーの id。SSO の場合、テナント id が接頭辞として付与される。 **/
    userId?: string
    /** コメントを残したユーザーのメールアドレス。 **/
    commenterEmail?: string
    /** コメントウィジェットに表示されるユーザー名。SSO の場合、displayName になることがある。 **/
    commenterName: string
    /** 生のコメントテキスト。 **/
    comment: string
    /** パース後のコメントテキスト。 **/
    commentHTML: string
    /** 外部コメント id。 **/
    externalId?: string
    /** 親コメントの id。 **/
    parentId?: string | null
    /** コメントが残された UTC 日時。 **/
    date: UTC_ISO_DateString
    /** 投票の合計カルマ（賛成 - 反対）。 **/
    votes: number
    votesUp: number
    votesDown: number
    /** ユーザーがコメント時にログインしていた、またはコメントが検証された、もしくはコメント作成時にセッションを検証していた場合は true。 **/
    verified: boolean
    /** コメントが検証された日時。 **/
    verifiedDate?: number
    /** モデレーターがコメントをレビュー済みとマークしたかどうか。 **/
    reviewed: boolean
    /** アバターの場所、または base64 エンコード。SSO でその値が渡された場合に限り base64 になる。 **/
    avatarSrc?: string
    /** コメントが手動または自動でスパムとしてマークされたか。 **/
    isSpam: boolean
    /** コメントが自動的にスパムと判定されたか。 **/
    aiDeterminedSpam: boolean
    /** コメント内に画像が含まれているか。 **/
    hasImages: boolean
    /** 「Most Relevant」ソート方向でのコメントのページ番号。 **/
    pageNumber: number
    /** 「Oldest First」ソート方向でのコメントのページ番号。 **/
    pageNumberOF: number
    /** 「Newest First」ソート方向でのコメントのページ番号。 **/
    pageNumberNF: number
    /** コメントが自動または手動で承認されたか。 **/
    approved: boolean
    /** コメント作成時のユーザーのロケールコード（形式: en_us）。 **/
    locale: string
    /** コメント内でパースに成功した @mentions。 **/
    mentions?: CommentUserMention[]
    /** コメントの発信ドメイン。 **/
    domain?: string
    /** このコメントに関連付けられたモデレーショングループ id のオプションリスト。 **/
    moderationGroupIds?: string[]|null
}
[inline-code-end]

コメント内でユーザーがタグ付けされた場合、その情報は `mentions` というリストに格納されます。そのリスト内の各オブジェクトは以下の構造を持ちます。

[inline-code-attrs-start title = 'Webhook メンションオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ユーザー id。SSO ユーザーの場合、テナント id が接頭辞として付く。 **/
    id: string
    /** @ 記号を含む、最終的な @mention タグのテキスト。 **/
    tag: string
    /** @ 記号を含む、元の @mention タグのテキスト。 **/
    rawTag: string
    /** タグ付けされたユーザーの種類。user = FastComments.com アカウント、sso = SSO ユーザー。 **/
    type: 'user'|'sso'
    /** ユーザーが通知をオプトアウトしている場合でも、これが true に設定されます。 **/
    sent: boolean
}
[inline-code-end]

#### 使用されるHTTPメソッド

**Create と Update はどちらも HTTP PUT を使用し、POST は使用しません！**

すべてのリクエストに ID が含まれているため、同じ Create または Update リクエストを繰り返しても、そちら側で新しいオブジェクトが作成されるべきではありません。

これはこれらの呼び出しが冪等（idempotent）であり、HTTP 仕様に従って PUT イベントであるべきことを意味します。