---
Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at vise en "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren går igennem det delvise {tenantId, urlId, commenterName}-indeks fra afterName fremad via $gt, uden $skip-omkostninger.

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nej |  |
| afterUserId | string | Nej |  |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PageUsersOfflineResponse.h)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
auto urlId = utility::string_t(U("article-456"));
boost::optional<utility::string_t> afterName = boost::optional<utility::string_t>(U("jane.doe@example.com"));
boost::optional<utility::string_t> afterUserId = boost::optional<utility::string_t>(U("user-789"));
api->getOfflineUsers(tenantId, urlId, afterName, afterUserId).then([](std::shared_ptr<PageUsersOfflineResponse> resp){
    auto result = resp ? resp : std::make_shared<PageUsersOfflineResponse>();
    (void)result;
});
[inline-code-end]

---