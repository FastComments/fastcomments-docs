---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| id | String | Oui |  |

## Réponse

Renvoie : `GetV2PageReactUsersResponse`

## Exemple

[inline-code-attrs-start title = 'get_v2_page_react_users Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_react_users(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetV2PageReactUsersParams = GetV2PageReactUsersParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/2026/space-flight-updates"),
        id: String::from("page-7a3f"),
        include_reaction_info: Some(true),
        limit: Some(100),
    };
    let response: GetV2PageReactUsersResponse = get_v2_page_react_users(configuration, params).await?;
    let _response = response;
    Ok(())
}
[inline-code-end]

---