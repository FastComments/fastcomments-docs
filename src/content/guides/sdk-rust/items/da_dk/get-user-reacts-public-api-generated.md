## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| post_ids | Vec<String> | Nej |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`UserReactsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/user_reacts_response.rs)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på get_user_reacts_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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