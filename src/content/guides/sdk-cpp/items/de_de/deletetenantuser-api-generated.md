## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|-------------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| deleteComments | string | Nein |  |
| commentDeleteMode | string | Nein |  |

## Antwort

Gibt zurück: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'deleteTenantUser Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> deleteComments = boost::optional<utility::string_t>(U("true"));
boost::optional<utility::string_t> commentDeleteMode = boost::optional<utility::string_t>(U("cascade"));
api->deleteTenantUser(tenantId, userId, deleteComments, commentDeleteMode)
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task){
        try {
            auto resp = task.get();
            if (!resp) resp = std::make_shared<APIEmptyResponse>();
            std::cout << "Tenant user deleted successfully\n";
        } catch (const std::exception &e) {
            std::cerr << "Failed to delete tenant user: " << e.what() << '\n';
        }
    });
[inline-code-end]

---