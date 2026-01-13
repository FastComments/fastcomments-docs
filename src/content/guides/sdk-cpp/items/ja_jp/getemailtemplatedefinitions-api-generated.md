## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |

## レスポンス

戻り値: [`GetEmailTemplateDefinitions_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateDefinitions_200_response.h)

## 例

[inline-code-attrs-start title = 'getEmailTemplateDefinitions の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantIdOpt = utility::string_t(U("my-tenant-123"));
api->getEmailTemplateDefinitions(tenantIdOpt.value())
.then([](pplx::task<std::shared_ptr<GetEmailTemplateDefinitions_200_response>> task) {
    try {
        auto resp = task.get();
        auto localCopy = std::make_shared<GetEmailTemplateDefinitions_200_response>(*resp);
        (void)localCopy;
    } catch (const std::exception& e) {
        auto fallback = std::make_shared<GetEmailTemplateDefinitions_200_response>();
        (void)fallback;
    }
});
[inline-code-end]

---