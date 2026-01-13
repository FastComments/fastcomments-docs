## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| deleteComments | bool | Nein |  |
| commentDeleteMode | string | Nein |  |

## Antwort

Gibt zurück: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteSSOUserAPIResponse.h)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für deleteSSOUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t ssoId = U("user@example.com");
boost::optional<bool> deleteComments = true;
boost::optional<utility::string_t> commentDeleteMode = U("anonymize");

api->deleteSSOUser(tenantId, ssoId, deleteComments, commentDeleteMode)
.then([](pplx::task<std::shared_ptr<DeleteSSOUserAPIResponse>> t){
    try {
        auto resp = t.get();
        auto respCopy = std::make_shared<DeleteSSOUserAPIResponse>(*resp);
        (void)respCopy;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---