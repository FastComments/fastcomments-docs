## Parameters

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | Yes |  |

## Response

Döndürür: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddPageAPIResponse.h)

## Örnek

[inline-code-attrs-start title = 'addPage Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto createData = CreateAPIPageData{};
createData.title = utility::string_t(U("Welcome Page"));
createData.url = utility::string_t(U("https://example.com/welcome"));
createData.description = boost::optional<utility::string_t>(utility::string_t(U("Landing page for new users")));

api->addPage(utility::string_t(U("my-tenant-123")), createData)
    .then([](std::shared_ptr<AddPageAPIResponse> response) {
        if (response && response->success) {
            // başarılı eklemeyi işle
        } else {
            // hatayı işle
        }
    });
[inline-code-end]