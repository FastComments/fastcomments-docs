---
Lister les pages d'un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.
Nécessite que `enableFChat` soit `true` dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent SSO sont filtrées en fonction des droits de groupe de l'utilisateur demandeur.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| cursor | string | Non |  |
| limit | int32_t | Non |  |
| q | string | Non |  |
| sortBy | PagesSortBy | Non |  |
| hasComments | bool | Non |  |

## Réponse

Renvoie: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> cursor = utility::string_t(U("cursor_abc"));
boost::optional<int32_t> limit = 50;
boost::optional<utility::string_t> q = utility::string_t(U("status:published"));
boost::optional<PagesSortBy> sortBy = PagesSortBy::NEWEST;
boost::optional<bool> hasComments = true;
api->getPagesPublic(tenantId, cursor, limit, q, sortBy, hasComments)
.then([](std::shared_ptr<GetPublicPagesResponse> resp){
    if (!resp) resp = std::make_shared<GetPublicPagesResponse>();
})
.wait();
[inline-code-end]

---