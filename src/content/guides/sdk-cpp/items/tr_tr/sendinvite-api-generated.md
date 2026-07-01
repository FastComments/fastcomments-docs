## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| fromName | string | Yes |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Örnek

[inline-code-attrs-start title = 'sendInvite Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> cc = utility::conversions::to_string_t("cc@example.com");
api->sendInvite(
    utility::conversions::to_string_t("my-tenant-123"),
    utility::conversions::to_string_t("invitee@example.com"),
    utility::conversions::to_string_t("John Doe")
).then([](std::shared_ptr<APIEmptyResponse> resp) {
    // başarılı daveti işle
});
[inline-code-end]