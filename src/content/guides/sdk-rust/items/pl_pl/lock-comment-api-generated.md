## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| broadcast_id | String | Tak |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład lock_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_lock_comment() -> Result<ApiEmptyResponse, Error> {
    let params: LockCommentParams = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        comment_id: "cmt-20240618-42".to_owned(),
        broadcast_id: "news/article/2024-06-18".to_owned(),
        sso: Some("user-12345-sso-token".to_owned()),
    };
    let response: ApiEmptyResponse = lock_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---