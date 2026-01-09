## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`UpdateUserBadge_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateUserBadge_200_response.h)

## Örnek

[inline-code-attrs-start title = 'deleteUserBadge Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t badgeId = utility::conversions::to_string_t("badge-456");
boost::optional<utility::string_t> traceId = boost::none;
api->deleteUserBadge(tenantId, badgeId)
.then([traceId](pplx::task<std::shared_ptr<UpdateUserBadge_200_response>> t) {
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<UpdateUserBadge_200_response>();
        return resp;
    } catch (const std::exception&) {
        return std::shared_ptr<UpdateUserBadge_200_response>();
    }
});
[inline-code-end]

---