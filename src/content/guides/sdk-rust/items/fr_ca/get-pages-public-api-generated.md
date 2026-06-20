Lister les pages d'un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons. Nécessite que `enableFChat` soit défini sur true dans la configuration personnalisée résolue pour chaque page. Les pages qui nécessitent SSO sont filtrées en fonction des accès de groupe de l'utilisateur demandeur.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| cursor | String | No |  |
| limit | i32 | No |  |
| q | String | No |  |
| sort_by | models::PagesSortBy | No |  |
| has_comments | bool | No |  |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetPagesPublicParams = GetPagesPublicParams {
    tenant_id: String::from("acme-corp-tenant"),
    cursor: Some(String::from("cursor_eyJwZl9pZCI6IjEyMyJ9")),
    limit: Some(50),
    q: Some(String::from("tag:release status:published")),
    sort_by: Some(models::PagesSortBy::CreatedAt),
    has_comments: Some(true),
};
let response: GetPublicPagesResponse = get_pages_public(&configuration, params).await?;
[inline-code-end]

---