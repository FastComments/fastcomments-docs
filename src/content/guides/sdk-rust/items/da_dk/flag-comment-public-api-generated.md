## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| is_flagged | bool | Ja |  |
| sso | String | Nej |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'flag_comment_public Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_flag() -> Result<FlagCommentPublic200Response, Error> {
    let params: FlagCommentPublicParams = FlagCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2026-03-25-8a7b6c".to_string(),
        is_flagged: true,
        sso: Some("sso-token-user-123".to_string()),
    };
    let response: FlagCommentPublic200Response = flag_comment_public(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---