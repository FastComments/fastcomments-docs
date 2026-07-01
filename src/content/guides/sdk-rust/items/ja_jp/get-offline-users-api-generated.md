ページ上の過去のコメント投稿者で、現在オンラインではないもの。displayNameでソートされます。  
このエンドポイントは、/users/online を使い切った後に、"Members" セクションを表示するために使用します。  
commenterName に対するカーソルページング: サーバーは部分的な {tenantId, urlId, commenterName} インデックスを afterName から $gt で前方に走査し、$skip のコストはかかりません。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## レスポンス

返り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## 例

[inline-code-attrs-start title = 'get_offline_users の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]