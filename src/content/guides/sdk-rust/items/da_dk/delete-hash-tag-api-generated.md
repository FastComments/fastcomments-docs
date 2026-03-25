## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tag | String | Ja |  |
| tenant_id | String | Nej |  |
| delete_hash_tag_request | models::DeleteHashTagRequest | Nej |  |

## Svar

Returnerer: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## Eksempel

[inline-code-attrs-start title = 'delete_hash_tag Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_tag(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params = DeleteHashTagParams {
        tag: "news/world-climate".to_owned(),
        tenant_id: Some("acme-corp-tenant".to_owned()),
        delete_hash_tag_request: None,
    };
    let response: FlagCommentPublic200Response = delete_hash_tag(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---