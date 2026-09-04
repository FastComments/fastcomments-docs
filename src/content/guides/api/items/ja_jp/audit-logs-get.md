[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

この API は `skip`、`limit`、`before`、`after` パラメータによるページングを使用します。AuditLogs はデフォルトで `100` 件ずつ返され、最大 `limit` は `200`、`when` と `id` の順序で並びます。

返される `100` 件のログごとにクレジットコストは `1` です。

デフォルトでは **最新の項目が最初** にリストされます。このため、`skip=0` からポーリングを開始し、最後に取得したレコードが見つかるまでページングできます。

あるいは、古い順にソートして、レコードがなくなるまでページングすることも可能です。

ソートは `order` を `ASC` または `DESC` に設定して行います。デフォルトは `DESC` です。

日付でのクエリは、ミリ秒単位のタイムスタンプとして `before` と `after` を使用して行えます。`before` と `after` は **含まれません**。どちらか一方だけでも使用できます。

## 人に何が起きたかを確認する

すべてのイベントは、実行者（`username`、`userId`、`ip`）と、実行対象を別々に記録します。`targetLabel` はそのオブジェクトの人間が読めるラベルで、例として `jsmith (jsmith@example.com)` のようになり、`targetId` はその ID です。`target` を使用すると、名前やメールアドレスは分かっても ID が分からない場合に、ラベルに対する大文字小文字を区別しない部分文字列検索ができます。

削除イベントはイベント時点のラベルを保持するため、基になるレコードが削除された後でも、削除されたユーザーやモデレーターを特定できます。

## 管理テナント

テナントが他のテナントを管理している場合、`includeManagedTenants=true` を設定すると、あなたのテナントと管理下のすべてのテナントからのイベントを 1 つのレスポンスで取得できます。返された各ログの `tenantId` が、どのテナントからのものかを示します。

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
    /** このリソース（例: User または Moderator）のイベントのみ。 **/
    resourceName?: string
    /** 影響を受けたオブジェクトの ID がこのもののイベントのみ。 **/
    targetId?: string
    /** 影響を受けたオブジェクトのラベルに対する大文字小文字を区別しない部分文字列検索。 **/
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