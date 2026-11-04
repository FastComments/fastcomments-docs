テナントのユーザー情報を一括取得します。userId を受け取り、User / SSOUser から表示情報を返します。
プレゼンスイベント経由で直後に現れたユーザーをコメントウィジェットが補完するために使用されます。
ページコンテキストはありません: プライバシーは一律に適用されます(非公開プロフィールはマスクされます)。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| ids | String | はい |  |

## レスポンス

戻り値: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## 例

[inline-code-attrs-start title = 'get_users_info の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]