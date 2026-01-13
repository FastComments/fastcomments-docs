[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

これは結果とコメントを結合するエンドポイントです。たとえば、製品の「最近の肯定的および否定的なコメント」チャートを作成するのに便利です。

値の範囲（両端含む）、1つ以上の質問、開始日（含む）で検索できます。

レスポンス構造は次のとおりです:

[inline-code-attrs-start title = 'QuestionResultsCombinedWithCommentsResponse 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SimpleComment {
    id: string
    commenterName: string
    userId?: string
    date: string
    commentHTML: string
}

interface SimpleQuestionResult {
    id: string
    value: number
}

interface CommentAndResult {
    comment: SimpleComment
    result: SimpleQuestionResult
}

interface QuestionResultsCombinedWithCommentsResponse {
    /** キャッシュから返されることがあるため、このデータが計算された日時を示す日付文字列。 **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

集計で使用できるクエリパラメータは次のとおりです:

[inline-code-attrs-start title = 'QuestionResultsCombineComments リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 1つ以上の質問の結果を集計できます。 **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

リクエストの例はこちら:

[inline-code-attrs-start title = 'QuestionResultsCombineComments の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

レスポンスの例:

[inline-code-attrs-start title = 'QuestionResultsCombineComments レスポンス例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "createdAt": "2023-08-30T00:00:00.000Z",
    "results": [
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-2",
                "commenterName": "Test",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 10
            }
        },
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-0",
                "commenterName": "Some Name",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 5
            }
        }
    ]
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsCombineComments レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### キャッシュとコストに関する注意事項

- `forceRecalculate` が指定されている場合、通常の `2` ではなくコストは常に `10` になります。
- キャッシュが期限切れになりデータが再計算される場合でも、`forceRecalculate` が指定されていなければコストは依然として一定の `2` です。
- これはキャッシュの利用を奨励するためです。

---