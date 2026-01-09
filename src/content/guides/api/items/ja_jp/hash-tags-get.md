[api-resource-header-start name = 'HashTag'; route = 'GET /api/v1/hash-tags'; creditsCost = 1; api-resource-header-end]

この API はページネーションを使用します。`page` クエリパラメータで指定します。ハッシュタグは `tag` によって並べ替えられ、1ページあたり `100` 件で返されます。

[inline-code-attrs-start title = 'HashTag cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/hash-tags?tenantId=demo&page=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'HashTag リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 取得するページ。0から開始。 **/
    page: number
}
[inline-code-end]

[inline-code-attrs-start title = 'HashTag レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTagsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** ハッシュタグ一覧！ **/
    hashTags: HashTag[]
}
[inline-code-end]

---