[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

この API は `skip`、`limit`、`before`、`after` パラメータによるページネーションを使用します。AuditLogs はデフォルトで `5000` 件のページで返され、最大 `limit` は `10000` です。`when` と `id` の順序で並びます。このエンドポイントは通常、履歴を一括で取得するために使用され、対話的にページングするためではないため、ページは大きくなります。

返される `100` 件のログごとにクレジットコストは `1` です。

デフォルトでは、**最新の項目が最初** のリストが返されます。これにより、`skip=0` からポーリングを開始し、取得した最後のレコードが見つかるまでページングできます。

あるいは、古い順にソートし、レコードがなくなるまでページングできます。

`order` を `ASC` または `DESC` に設定することでソートできます。デフォルトは `DESC` です。

`before` と `after` をミリ秒単位のタイムスタンプとして使用することで日付でのクエリが可能です。`before` と `after` は含まれません（排他的）で、どちらか単独でも使用できます。

## 人物に何が起きたかを確認する

各イベントは、実行者（`username`、`userId`、`ip`）と、実行対象を別々に記録します。`targetLabel` はそのオブジェクトの人間が読めるラベルで、例として `jsmith (jsmith@example.com)` のようになります。`targetId` はその ID です。人物の名前やメールアドレスは分かっているが ID が分からない場合は、`target` を使用してラベルに対する大文字小文字を区別しない部分文字列検索を行います。

削除はイベント時点のラベルを取得するため、基になるレコードが削除された後でも、削除されたユーザーやモデレーターを特定できます。

## 管理テナント

テナントが他のテナントを管理している場合、`includeManagedTenants=true` を設定すると、あなたのテナントと管理下のすべてのテナントからのイベントを 1 つのレスポンスで返します。返される各ログの `tenantId` がどのテナントからのものかを示します。

[inline-code-attrs-start title = 'AuditLog cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** 最大 10000。デフォルトは 5000。 **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** このユーザー名で実行されたイベントのみ。 **/
    username?: string
    /** この IP アドレスからのイベントのみ。 **/
    ip?: string
    /** このタイプのイベントのみ。 **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** このリソース（例: ユーザーまたはモデレーター）のイベントのみ。 **/
    resourceName?: string
    /** 影響を受けたオブジェクトがこの ID を持つイベントのみ。 **/
    targetId?: string
    /** 影響を受けたオブジェクトのラベルに対する大文字小文字を区別しない部分文字列一致。 **/
    target?: string
    /** このテナントが管理するテナントからのイベントも返す。 **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 応答構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** 失敗時に含まれます。 **/
    reason?: string
    /** ログです！ **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---