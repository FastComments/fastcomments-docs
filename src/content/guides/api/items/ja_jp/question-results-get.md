[api-resource-header-start name = 'QuestionResults'; route = 'GET /api/v1/question-results'; creditsCost = 1; api-resource-header-end]

このルートは一度に最大1000件の`QuestionResults`オブジェクトをページネーションされた状態で返します。コストは100件ごとに1です。これらは
`createdAt`で昇順にソートされます。さまざまなパラメータでフィルタできます。

[inline-code-attrs-start title = 'QuestionResults の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** ページネーション用。0から始まります。 **/
    skip?: number
    /** ページネーション用。 **/
    limit?: number
    /** 特定のページからの結果を取得します。 **/
    urlId?: string
    /** 特定のユーザーからの結果を取得します。 **/
    userId?: string
    startDate?: string | number
    questionId?: string | string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResults レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    questionResults: QuestionResults[]
}
[inline-code-end]