Past commenters on the page who are NOT currently online. Sorted by displayName.  
Utilisez ceci après avoir épuisé /users/online pour afficher une section « Members ».  
Pagination par curseur sur commenterName : le serveur parcourt la partie {tenantId, urlId, commenterName} index à partir de afterName vers l’avant via $gt, sans coût $skip.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| options | const GetOfflineUsersOptions& | Oui |  |

## Response

Retourne : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Exemple

[inline-code-attrs-start title = 'Exemple getOfflineUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("page-456");
GetOfflineUsersOptions options;
options.limit = boost::optional<int>(50);
options.includeDetails = boost::optional<bool>(true);

api->getOfflineUsers(tenantId, urlId, options)
    .then([](pplx::task<std::shared_ptr<PageUsersOfflineResponse>> t) {
        try {
            auto response = t.get();
        } catch (const std::exception&) {
        }
    });
[inline-code-end]