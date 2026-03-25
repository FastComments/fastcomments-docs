## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| post_ids | Vec<String> | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_reacts_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_user_reacts_public Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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