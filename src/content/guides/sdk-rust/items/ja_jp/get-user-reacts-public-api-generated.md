## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| post_ids | Vec<String> | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/user_reacts_response.rs)

## 例

[inline-code-attrs-start title = 'get_user_reacts_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetUserReactsPublicParams = GetUserReactsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: Some(vec![
            "news/article-123".to_string(),
            "blog/post-456".to_string(),
        ]),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
    };
    let response: UserReactsResponse = get_user_reacts_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---