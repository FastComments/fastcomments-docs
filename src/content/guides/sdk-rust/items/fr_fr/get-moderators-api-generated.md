## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenant_id | String | Oui |  |
| skip | f64 | Non |  |

## Réponse

Renvoie : [`GetModerators200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_moderators'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetModeratorsParams = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(10.0),
    };
    let _moderators: GetModerators200Response = get_moderators(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---