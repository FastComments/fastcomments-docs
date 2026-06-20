## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| skip | f64 | Non |  |

## Réponse

Retourne : [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_moderators'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_moderators(configuration: &configuration::Configuration) -> Result<GetModeratorsResponse, Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let response: GetModeratorsResponse = get_moderators(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---