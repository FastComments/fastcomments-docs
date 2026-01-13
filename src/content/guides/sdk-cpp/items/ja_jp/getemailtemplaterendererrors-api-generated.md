## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| skip | double | No |  |

## レスポンス

返却値: [`GetEmailTemplateRenderErrors_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetEmailTemplateRenderErrors_200_response.h)

## 例

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("email-template-789");
boost::optional<double> skip{10.0};
auto defaultResp = std::make_shared<GetEmailTemplateRenderErrors_200_response>();
api->getEmailTemplateRenderErrors(tenantId, id, skip).then([defaultResp](pplx::task<std::shared_ptr<GetEmailTemplateRenderErrors_200_response>> t){
    try {
        auto resp = t.get();
        if(!resp) resp = defaultResp;
        std::wcout << (resp ? U("Received render errors response\n") : U("No response\n"));
    } catch(...) {
        std::wcout << U("Failed to retrieve render errors\n");
    }
});
[inline-code-end]