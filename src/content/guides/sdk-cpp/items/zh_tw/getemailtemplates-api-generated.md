## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 回應

回傳：[`GetEmailTemplates_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplates_200_response.h)

## 範例

[inline-code-attrs-start title = 'getEmailTemplates 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getEmailTemplates(tenantId, skip).then([](pplx::task<std::shared_ptr<GetEmailTemplates_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            auto copy = std::make_shared<GetEmailTemplates_200_response>(*resp);
        }
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]

---