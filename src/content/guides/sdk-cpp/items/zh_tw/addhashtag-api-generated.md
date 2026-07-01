## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createHashTagBody | CreateHashTagBody | Yes |  |

## 回應

返回: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateHashTagResponse.h)

## 範例

[inline-code-attrs-start title = 'addHashTag 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
CreateHashTagBody request;
request.tag = utility::conversions::to_string_t("feature-request");
request.description = utility::conversions::to_string_t("Requests for new features");
request.relatedUrl = boost::optional<utility::string_t>(utility::conversions::to_string_t("https://example.com/feature-request"));

api->addHashTag(utility::conversions::to_string_t("my-tenant-123"), request)
    .then([](std::shared_ptr<CreateHashTagResponse> resp) {
        (void)resp;
    });
[inline-code-end]