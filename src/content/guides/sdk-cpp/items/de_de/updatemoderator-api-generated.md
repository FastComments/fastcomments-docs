---
## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateModeratorBody | UpdateModeratorBody | Ja |  |

## Antwort

Gibt zurück: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für updateModerator'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t moderatorId = U("moderator-456");
auto bodyPtr = std::make_shared<UpdateModeratorBody>();
bodyPtr->email = utility::string_t(U("mod.jane@example.com"));
bodyPtr->displayName = boost::optional<utility::string_t>(utility::string_t(U("Jane Moderator")));
bodyPtr->enabled = boost::optional<bool>(true);
api->updateModerator(tenantId, moderatorId, *bodyPtr).then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "Moderator updated successfully" << std::endl;
        }
    } catch (const std::exception &e) {
        std::cerr << "Update failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---