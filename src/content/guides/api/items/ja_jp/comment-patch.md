[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントは単一のコメントを更新する機能を提供します。

注意:

- このAPIは、必要に応じてコメントウィジェットを「ライブ」で更新できます（これにより基本の`creditsCost`は`1`から`2`に増加します）。
  - これによりコメントをページ間で「ライブ」に移行する（`urlId`を変更する）ことができます。
  - 移行はページが事前計算されるためCPU負荷が高く、追加で`2`クレジットがかかります。
- 作成APIと異なり、このAPIはメールアドレスが提供されてもユーザーオブジェクトを自動的に作成しません。
- このAPIで更新されたコメントは、必要に応じてまだスパムチェックの対象にできます。
- カスタマイズルール管理ページで設定された最大コメント長などの設定はここでも適用されます。
- ユーザーにコメントテキストの更新を許可するには、リクエストボディに`comment`を指定するだけで構いません。結果の`commentHTML`を生成します。
  - `comment`と`commentHTML`の両方を定義した場合、自動的にHTMLは生成されません。
  - ユーザーが新しいテキストにメンションやハッシュタグを追加した場合、それらは`POST` APIと同様に処理されます。
- コメントの`commenterEmail`を更新する場合は、`userId`も指定するのが望ましいです。そうでない場合、そのメールアドレスのユーザーがあなたのテナントに属していることを確認する必要があります。そうでなければリクエストは失敗します。  
- 対象のコメントがロックされている場合（`isLocked: true`）、リクエストは`code: 'locked'`で拒否されます。まずコメントのロックを解除して更新し、必要なら再度ロックしてください。


[inline-code-attrs-start title = '最小限のコメントPATCH cURL例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'コメント PATCH リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** 更新を行うユーザー。必要に応じて、そのユーザーがコメントを編集できるかをチェックするために使用できます。 **/
    contextUserId?: string
	/** 新しいコメントがスパムに見えるかをチェックするかどうか。 **/
    doSpamCheck?: 'true' | 'false'
	/** 同じurlIdを持つコメントウィジェットのインスタンスを見ているユーザーにコメントを「ライブ」で表示するかどうか。注意: クレジットコストが1から2に倍増します。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'コメント PATCH レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---