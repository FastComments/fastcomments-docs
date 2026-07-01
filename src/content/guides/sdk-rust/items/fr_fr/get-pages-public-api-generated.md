Lister les pages d'un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.  
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.  
Les pages nécessitant SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenant_id | String | Oui |  |
| cursor | String | Non |  |
| limit | i32 | Non |  |
| q | String | Non |  |
| sort_by | models::PagesSortBy | Non |  |
| has_comments | bool | Non |  |

## Response

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Exemple

[inline-code-attrs-start title = 'get_pages_public Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---