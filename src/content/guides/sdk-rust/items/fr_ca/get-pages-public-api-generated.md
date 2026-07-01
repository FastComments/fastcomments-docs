Liste les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles. Nécessite `enableFChat` à vrai dans la configuration personnalisée résolue pour chaque page. Les pages qui nécessitent SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## Response

Retourne : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]