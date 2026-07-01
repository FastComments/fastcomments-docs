## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| id | String | Oui |  |
| edit_key | String | Non |  |

## Réponse

Renvoie : [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_delete_response.rs)

## Exemple

[inline-code-attrs-start title = 'delete_vote Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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