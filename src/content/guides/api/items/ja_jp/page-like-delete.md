[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

現在のユーザーのページへの「いいね」を削除します。ユーザーがページに「いいね」していない場合、リクエストはコード `not-liked` で成功し、カウントは変更されません。

[inline-code-attrs-start title = 'ページのいいね解除 cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'ページのいいね解除 リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページのいいね解除 レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' はユーザーがページにいいねしていなかったときです。失敗時に含まれます。 **/
    code?: 'not-liked' | string
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]

---