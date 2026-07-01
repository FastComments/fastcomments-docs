## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenant_id | String | Ja |  |
| id | String | Ja |  |
| edit_key | String | Nein |  |

## Antwort

Rückgabe: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Beispiel

[inline-code-attrs-start title = 'delete_vote Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = DeleteVoteParams {
        tenant_id: "acme-corp".to_string(),
        id: "vote-12345".to_string(),
        edit_key: Some("edit-key-abc".to_string()),
    };
    let _response: VoteDeleteResponse = delete_vote(&configuration, params).await?;
    Ok(())
}
[inline-code-end]