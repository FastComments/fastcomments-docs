[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントはコメントを作成する機能を提供します。

一般的な使用例はカスタムUI、統合、またはインポートです。

注意事項:

- このAPIは、必要に応じてコメントウィジェットを「ライブ」で更新できます（これにより `creditsCost` が `1` から `2` に増加します）。
- メールアドレスが提供されると、このAPIはシステム内にユーザーオブジェクトを自動的に作成します。
- 異なるメールアドレスで同じユーザー名のコメントを2件保存しようとすると、2つ目のコメントでエラーになります。 
- `parentId` を指定していて、子コメントの `notificationSentForParent` が false の場合、**親コメントに対して通知を送信します**。これは毎時間行われます（送信されるメール数を減らすために通知をまとめて送信します）。
- ユーザー作成時に歓迎メールを送信したり、コメント検証メールを送信したい場合は、クエリパラメータで `sendEmails` を `true` に設定してください。
- このAPIで作成されたコメントは、管理アプリの Analytics および Moderation ページに表示されます。
- 設定がオンになっている場合、コメント投稿者の名前やコメント本文の「悪い言葉」はマスクされます。
- このAPIで作成されたコメントは、必要に応じてスパムチェックを受けることができます。
- Customization Rule 管理ページで設定された最大コメント長などの設定は、ここにも適用されます。

コメントウィジェットに表示されるために送信するのに必要な最小データは、次のとおりです:

[inline-code-attrs-start title = '最小限のコメントPOST cURL例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

より現実的なリクエストは次のようになります:

[inline-code-attrs-start title = 'コメントPOSTのcURL例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'コメントPOSTリクエストの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** 同じ urlId を持つコメントウィジェットのインスタンスを閲覧しているユーザーに対してコメントを「ライブ」で表示するかどうか。注意：クレジットコストが1から2に倍増します。 **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'コメントPOSTレスポンスの構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 作成されたコメント。 **/
    comment?: Comment
    /** 関連するユーザー（既に存在していたかどうかは問わない）。 **/
    user?: User
}
[inline-code-end]