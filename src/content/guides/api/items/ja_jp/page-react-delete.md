[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

ページから現在のユーザーのリアクションの1つを削除します。ユーザーがそのリアクションを追加していなかった場合、リクエストはコード `no-react` で成功し、カウントは変更されません。

[inline-code-attrs-start title = 'ページリアクション削除 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクション削除 リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** リアクション ID。 **/
    id: string
    /** URI エンコードされた JSON の SSO オブジェクト。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクション削除 レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** ユーザーがこのリアクションを追加していなかった場合は 'no-react'。それ以外は失敗時に含まれます。 **/
    code?: 'no-react' | string
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]