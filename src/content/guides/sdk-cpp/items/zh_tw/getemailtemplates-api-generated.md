## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | double | 否 |  |

## 回應

返回：[`GetEmailTemplatesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetEmailTemplatesResponse.h)

## 範例

[inline-code-attrs-start title = 'getEmailTemplates 範例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = utility::string_t(U("my-tenant-123"));
boost::optional<double> skip = 10.0;
api->getEmailTemplates(tenantId, skip)
    .then([](std::shared_ptr<GetEmailTemplatesResponse> resp) {
        (void)resp;
    });
[inline-code-end]

---