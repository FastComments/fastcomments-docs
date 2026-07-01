## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetCommentsOptions& | Yes |  |

## Réponse

Retourne : [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getComments'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCommentsOptions options;
options.page = 1;
options.limit = 50;
options.sort = U("newest");
options.authorEmail = boost::optional<utility::string_t>(U("user@example.com"));
api->getComments(tenantId, options).then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> task) {
    try {
        auto response = task.get();
        // utiliser la réponse au besoin
    } catch (const std::exception& e) {
        // gérer l'erreur
    }
});
[inline-code-end]