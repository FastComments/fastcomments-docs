## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| user_id | String | Nein |  |
| anon_user_id | String | Nein |  |

## Antwort

Gibt zurück: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_response.rs)

## Beispiel

[inline-code-attrs-start title = 'un_flag_comment Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn unflag_example() -> Result<FlagCommentResponse, Error> {
    let params: UnFlagCommentParams = UnFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "comment-98765".to_string(),
        user_id: Some("user-42".to_string()),
        anon_user_id: None,
    };
    let response: FlagCommentResponse = un_flag_comment(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---