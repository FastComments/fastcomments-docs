Lister les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages nécessitant SSO sont filtrées en fonction de l'accès par groupe de l'utilisateur requérant.

## Parameters

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| cursor | String | Non |  |
| limit | i32 | Non |  |
| q | String | Non |  |
| sort_by | models::PagesSortBy | Non |  |
| has_comments | bool | Non |  |

## Response

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple get_pages_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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