## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| post_ids | Vec<String> | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_reacts_public_200_response.rs)

## 例

[inline-code-attrs-start title = 'get_user_reacts_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<GetUserReactsPublic200Response, Error> {
    let params: GetUserReactsPublicParams = GetUserReactsPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        post_ids: Some(vec![
            "news/article-123".to_string(),
            "blog/post-456".to_string(),
        ]),
        sso: Some("john.doe@acme.com".to_string()),
    };
    let response: GetUserReactsPublic200Response = get_user_reacts_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---