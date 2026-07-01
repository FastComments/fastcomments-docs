## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| user_id | String | Nein |  |
| anon_user_id | String | Nein |  |

## Antwort

Rückgabe: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## Beispiel

[inline-code-attrs-start title = 'flag_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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