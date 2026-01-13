[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

ここでは結果の集計が行われます。

集計のレスポンス構造は以下の通りです:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** 現在のデータポイント（日時バケットまたはページ）における値ごとの出現回数を示すマップ。 **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** 注：timeBucket が指定されていない場合は null になります。 **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** 集計された結果の総数。 **/
    total: number
    /** 結果として得られる加重平均。小数で、通常は小数点以下2桁以内です。 **/
    average: number
    /** このデータが計算された日時を表す日付文字列（キャッシュから取得される可能性があるため）。 **/
    createdAt: string
}
[inline-code-end]

集計で利用できるクエリパラメータは以下の通りです:

[inline-code-attrs-start title = 'QuestionResultsAggregation のリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 結果は1つまたは複数の質問で集計できます。 **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** 特定のページで集計します。 **/
    urlId?: string
    /** 特定のユーザーで集計します。 **/
    userId?: string
    /** 今すぐ再計算してキャッシュを更新します。 **/
    forceRecalculate?: boolean
}
[inline-code-end]

リクエストの例:

[inline-code-attrs-start title = 'QuestionResultsAggregation の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

レスポンス例:

[inline-code-attrs-start title = 'QuestionResultsAggregation のレスポンス例'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    {
        "average": 8.33,
        "countsByValue": {
            "5": 1,
            "10": 2
        },
        "createdAt": "2023-08-30T00:00:00.000Z",
        "dataByUrlId": {
            "some-page": {
                "total": 3,
                "v": {
                    "5": 1,
                    "10": 2
                }
            }
        },
        "total": 3
    }
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsAggregation のレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### パフォーマンスに関する注意

- キャッシュミスの場合、集計は通常100万件の結果あたり約5秒かかります。
- それ以外では、リクエストは定数時間で処理されます。

### キャッシュとコストに関する注意

- `forceRecalculate` が指定されている場合、通常の `2` の代わりにコストは常に `10` になります。
- キャッシュが期限切れになりデータが再計算されても、`forceRecalculate` が指定されていなければコストは依然として定数の `2` です。キャッシュの有効期限は集計されるデータセットのサイズに応じて決まり（30秒〜5分の間で変動します）。
- これはキャッシュの利用を促すための仕組みです。

---