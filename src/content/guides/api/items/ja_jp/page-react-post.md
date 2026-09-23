[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

現在のユーザーとしてページにリアクションを追加します。各リアクション ID はユーザーごとに一度だけ追加可能です。再度追加すると `already-reacted` コードで成功し、カウントは変わりません。ユーザーは同じページに対して複数の異なるリアクションを追加できます。

リアクション ID は利用者が選択し、最大 36 文字まで設定できます。ページがまだ存在しない場合は作成されます。`title` を渡すことでページのタイトルを設定または更新できます。

[inline-code-attrs-start title = 'ページリアクション cURL 例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクション リクエスト構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** リアクション ID（最大 36 文字）。 **/
    id: string
    /** ページのタイトルを設定します。 **/
    title?: string
    /** SSO オブジェクトの URI エンコードされた JSON。匿名ユーザーの場合は省略してください。 **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'ページリアクション レスポンス構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** ユーザーがすでにこのリアクションを追加していた場合は 'already-reacted'、ID が 36 文字を超える場合は 'react-id-too-long'（HTTP 422）となります。その他の失敗時はこのフィールドに含まれます。 **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** 失敗時に含まれます。 **/
    reason?: string
}
[inline-code-end]