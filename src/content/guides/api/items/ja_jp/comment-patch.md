[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

この API エンドポイントは単一のコメントを更新する機能を提供します。

Notes:

- 必要に応じて、この API はコメントウィジェットを「ライブ」で更新できます（これにより基本の `creditsCost` が `1` から `2` に増加します）。
  - これにより、ページ間のコメント移行を「ライブ」で行うことができます（`urlId` の変更）。
  - この移行はページが事前計算され CPU 負荷が高いため、追加で `2` クレジットがかかります。
- 作成 API と異なり、この API はメールが提供されても当社システム内にユーザーオブジェクトを自動で作成しません。
- この API を介して更新されたコメントでも、必要に応じてスパムチェックが可能です。
- カスタマイズルール管理ページで設定された最大コメント長などの設定はここにも適用されます。
- ユーザーがコメント本文を更新できるようにするには、リクエストボディに `comment` を指定するだけです。結果の `commentHTML` を生成します。
  - `comment` と `commentHTML` の両方を定義した場合、HTML は自動生成されません。
  - ユーザーが新しいテキストにメンションやハッシュタグを追加した場合でも、`POST` API と同様に処理されます。
- コメントの `commenterEmail` を更新する際は、`userId` も指定するのが最善です。そうでない場合は、このメールアドレスのユーザーがあなたのテナントに属していることを確認する必要があります。そうでなければリクエストは失敗します。  


[inline-code-attrs-start title = '最小のコメント PATCH cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** 更新を行うユーザー。必要に応じて、そのユーザーがコメントを編集できるかを確認するために使用できます。  **/
    contextUserId?: string
	/** 新しいコメントがスパムに見えるかどうかをチェックするか？  **/
    doSpamCheck?: 'true' | 'false'
	/** 同じ urlId を持つコメントウィジェットのインスタンスを閲覧しているユーザーに対して、コメントを「ライブ」で表示するかどうか。NOTE: クレジットコストが1から2に倍増します。 **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'コメント PATCH レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれる。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** 失敗時に含まれる。 **/
    reason?: string
}
[inline-code-end]

---