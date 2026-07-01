## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| tag | string | Yes |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Yes |  |

## תשובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת deleteHashTag'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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