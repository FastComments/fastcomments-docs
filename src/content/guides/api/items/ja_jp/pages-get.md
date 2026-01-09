[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

現在、アカウントに関連付けられたページをすべて取得するか（または `/by-url-id` 経由で単一のページを取得する）ことのみ可能です。より細かい検索を希望する場合は、[お問い合わせください](https://fastcomments.com/auth/my-account/help)。 

[inline-code-attrs-start title = 'Pages の cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Pages リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Pages レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### 役立つヒント

`Comment` API では `urlId` が必要です。まず `Pages` API を呼び出して、利用可能な `urlId` の値がどのようなものかを確認できます。

---