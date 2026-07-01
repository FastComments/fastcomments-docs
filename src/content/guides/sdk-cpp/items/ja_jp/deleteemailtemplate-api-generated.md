## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 例

[inline-code-attrs-start title = 'deleteEmailTemplate の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::conversions::to_string_t("my-tenant-123");
auto templateId = utility::conversions::to_string_t("welcome-email-template");
boost::optional<utility::string_t> optTenantId = tenantId;
boost::optional<utility::string_t> optTemplateId = templateId;
api->deleteEmailTemplate(optTenantId.value(), optTemplateId.value())
    .then([](pplx::task<std::shared_ptr<APIEmptyResponse>> t) {
        try { t.get(); }
        catch (const std::exception&) {}
    });
[inline-code-end]