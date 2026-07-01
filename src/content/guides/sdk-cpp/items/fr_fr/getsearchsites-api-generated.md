## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetSearchSitesOptions& | Yes |  |

## Réponse

Renvoie : [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationSiteSearchResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchSites'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
GetSearchSitesOptions options;
options.query = boost::make_optional(utility::string_t(U("example query")));
options.limit = boost::make_optional(25);

api->getSearchSites(tenantId, options)
   .then([](pplx::task<std::shared_ptr<ModerationSiteSearchResponse>> t) {
       try {
           auto response = t.get();
           auto respPtr = std::make_shared<ModerationSiteSearchResponse>(*response);
           for (const auto& site : respPtr->sites) {
               // logique de traitement
           }
       } catch (const std::exception&) {
           // gestion des erreurs
       }
   });
[inline-code-end]