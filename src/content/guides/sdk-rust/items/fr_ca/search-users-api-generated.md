## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| username_starts_with | String | Non |  |
| mention_group_ids | Vec<String> | Non |  |
| sso | String | Non |  |
| search_section | String | Non |  |

## Réponse

Retourne : [`SearchUsersResult`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/search_users_result.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple search_users'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_search() -> Result<(), Error> {
    let params = SearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        username_starts_with: Some("john".to_string()),
        mention_group_ids: Some(vec!["group1".to_string(), "group2".to_string()]),
        sso: Some("sso-provider".to_string()),
        search_section: Some("comments".to_string()),
    };
    let _result: SearchUsersResult = search_users(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---