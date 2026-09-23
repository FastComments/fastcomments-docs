---
[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

ページ上の各リアクションのカウントと、現在のユーザーが追加したリアクションを返します。

[inline-code-attrs-start title = 'ページリアクション cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクション リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクション レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** 失敗時に含まれます。 **/
    code?: string
    /** 失敗時に含まれます。 **/
    reason?: string
    /** リアクション ID ごとのカウント。例: {"heart": 12, "laugh": 3}。ページにリアクションがない場合は設定されません。 **/
    counts?: Record<string, number>
    /** リクエストを行ったユーザーが追加したリアクション ID。追加がない場合は設定されません。 **/
    reactedIds?: string[]
}
[inline-code-end]

---