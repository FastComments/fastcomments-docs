## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Yes |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Eksempel

[inline-code-attrs-start title = 'deleteHashTag Eksempel'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-001");
auto tag = utility::conversions::to_string_t("news");
DeleteHashTagRequestBody requestBody;
requestBody.userId = utility::conversions::to_string_t("user-42");
requestBody.reason = boost::optional<utility::string_t>(utility::conversions::to_string_t("User request"));
api->deleteHashTag(tenantId, tag, requestBody)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
    });
[inline-code-end]