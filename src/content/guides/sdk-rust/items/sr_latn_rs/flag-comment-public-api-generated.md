## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| is_flagged | bool | Da |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Primer

[inline-code-attrs-start title = 'flag_comment_public Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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