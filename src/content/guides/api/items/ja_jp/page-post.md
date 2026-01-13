[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

このAPIエンドポイントはページを作成する機能を提供します。

一般的なユースケースはアクセス制御です。

注意事項:

- もしあなたがコメントスレッドにコメントしたり、`Comment` を作成するためにAPIを呼び出した場合、既に `Page` オブジェクトが作成されています！同じ `urlId` をコメントウィジェットに渡したものを使って、`/by-url-id` の `Page` ルートから取得してみてください。
- `Page` 構造にはいくつかの**計算された**値が含まれます。
  現在は `commentCount` と `rootCommentCount` です。
  これらは自動で設定され、APIから設定することはできません。設定しようとするとAPIはエラーを返します。

[inline-code-attrs-start title = 'Page POST cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page POST リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** 失敗時に含まれます。 **/
    reason?: string
    /** 作成されたページ。 **/
    page?: Page
}
[inline-code-end]

---