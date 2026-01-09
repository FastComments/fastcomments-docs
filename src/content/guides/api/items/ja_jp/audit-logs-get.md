[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

この API はページネーションを使用します。ページネーションは `skip`、`before`、`after` パラメータで行います。AuditLogs は `1000` 件ずつのページで返され、`when` と `id` によって順序付けられます。

毎回 `1000` 件のログを取得するにはクレジット `10` が必要です。

デフォルトでは、**最新の項目が先に来る**リストが返されます。これにより、`skip=0` からポーリングを開始し、最後に消費したレコードが見つかるまでページネーションできます。

あるいは、古い順でソートして、レコードがなくなるまでページネーションすることもできます。

ソートは `order` を `ASC` または `DESC` のいずれかに設定して行います。デフォルトは `ASC` です。

日付でのクエリは、ミリ秒単位のタイムスタンプとしての `before` と `after` で可能です。`before` と `after` は含まれません。

[inline-code-attrs-start title = 'AuditLog cURL の例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** ログ本体です！ **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---