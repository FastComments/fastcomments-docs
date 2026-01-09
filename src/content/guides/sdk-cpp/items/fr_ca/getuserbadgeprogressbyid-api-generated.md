## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |

## Réponse

Renvoie : [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserBadgeProgressById'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("badge-abc-456");
boost::optional<utility::string_t> preferredLocale = boost::optional<utility::string_t>(U("en-US"));

api->getUserBadgeProgressById(tenantId, id)
.then([=](std::shared_ptr<GetUserBadgeProgressById_200_response> resp){
    if(!resp) return;
    auto copy = std::make_shared<GetUserBadgeProgressById_200_response>(*resp);
    (void)copy;
});
[inline-code-end]

---