## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Hayır |  |

## Yanıt

Döndürür: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSubscriptionsAPIResponse.h)

## Örnek

[inline-code-attrs-start title = 'getSubscriptions Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenant = utility::conversions::to_string_t("my-tenant-123");
boost::optional<utility::string_t> user = utility::conversions::to_string_t("user@example.com");

api->getSubscriptions(tenant, user).then(
    [](pplx::task<std::shared_ptr<GetSubscriptionsAPIResponse>> t) {
        try {
            auto response = t.get();
            // yanıtı işle
        } catch (const std::exception& e) {
            // hatayı işle
        }
    }
);
[inline-code-end]

---