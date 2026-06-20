---
テナント向けの一括ユーザー情報。userIds を受け取り、User / SSOUser から表示情報を返します。
コメントウィジェットが、プレゼンスイベントで新たに出現したユーザーを補強するために使用します。
ページコンテキストがないため、プライバシーは一律に適用されます（非公開プロフィールはマスクされます）。

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
let params: GetUsersInfoParams = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "alice@example.com,bob@example.com,carol@example.com".to_string(),
    page_size: Some(100),
};
let users_response: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]

---