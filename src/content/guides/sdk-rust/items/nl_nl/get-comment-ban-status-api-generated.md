## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| sso | String | Nee |  |

## Response

Retourneert: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_ban_status_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_comment_ban_status Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetCommentBanStatusParams = GetCommentBanStatusParams {
        comment_id: String::from("cmt-9f8b7a6e-4d3c-11ee-8c99-0242ac120002"),
        sso: Some(String::from("acme-corp-tenant")),
    };
    let response: GetCommentBanStatusResponse = get_comment_ban_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---