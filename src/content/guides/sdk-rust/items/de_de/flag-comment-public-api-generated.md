## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| is_flagged | bool | Ja |  |
| sso | String | Nein |  |

## Antwort

Gibt zurück: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Beispiel

[inline-code-attrs-start title = 'flag_comment_public Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_flag_comment() -> Result<(), Error> {
    let params: FlagCommentPublicParams = FlagCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("comment-89b3"),
        is_flagged: true,
        sso: Some(String::from("sso-uid-7a2f")),
    };

    let _response: ApiEmptyResponse = flag_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---