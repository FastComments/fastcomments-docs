## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| post_ids | Vec<String> | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/user_reacts_response.rs)

## Primjer

[inline-code-attrs-start title = 'get_user_reacts_public Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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