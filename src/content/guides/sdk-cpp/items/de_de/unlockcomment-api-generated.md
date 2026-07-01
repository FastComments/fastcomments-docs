## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Ja |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Beispiel

[inline-code-attrs-start title = 'unLockComment Beispiel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto commentId = utility::conversions::to_string_t("cmt-456789");
auto broadcastId = utility::conversions::to_string_t("broadcast-001");
boost::optional<utility::string_t> sso = utility::conversions::to_string_t("john.doe@example.com");

api->unLockComment(tenantId, commentId, broadcastId, sso).then([](pplx::task<std::shared_ptr<APIEmptyResponse>> task) {
    try {
        auto response = task.get();
    } catch (const std::exception&) {
    }
});
[inline-code-end]