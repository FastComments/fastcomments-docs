Past commenters on the page who are NOT currently online. Sorted by displayName.  
Commentateurs précédents sur la page qui ne sont PAS actuellement en ligne. Triés par displayName.

Use this after exhausting /users/online to render a "Members" section.  
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Membres ».

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Pagination par curseur sur commenterName : le serveur parcourt le fragment {tenantId, urlId, commenterName} à partir de afterName vers l’avant via $gt, sans coût $skip.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Response

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'Exemple get_offline_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]