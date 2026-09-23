[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

現在のユーザーとしてページに「いいね」します。各ユーザーはページに一度だけ「いいね」できます。再度「いいね」すると、コード `already-liked` が返され、カウントは変わりません。

ページがまだ存在しない場合は作成されます。`title` を渡すことでページのタイトルを設定または更新できます。

[inline-code-attrs-start title = 'ページいいね cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'ページいいねリクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** ページのタイトルを設定します。 **/
    title?: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページいいねレスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** ユーザーがすでにページに「いいね」していた場合は 'already-liked'。それ以外は失敗時に含まれます。 **/
    code?: 'already-liked' | string
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]