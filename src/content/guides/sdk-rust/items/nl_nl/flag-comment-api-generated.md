## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| user_id | String | Nee |  |
| anon_user_id | String | Nee |  |

## Respons

Retourneert: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'flag_comment Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = FlagCommentParams {
        tenant_id: "acme-corp".to_string(),
        id: "comment-9876".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let _response = flag_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---