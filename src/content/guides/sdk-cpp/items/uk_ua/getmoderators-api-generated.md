## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | double | No |  |

## Відповідь

Повертає: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModeratorsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerators'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getModerators(tenantId, skip).then([](pplx::task<std::shared_ptr<GetModeratorsResponse>> t){
    auto response = t.get();
});
[inline-code-end]