## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| skip | double | לא |  |

## תגובה

מחזיר: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetModeratorsResponse.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getModerators'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<double> skip = 10.0;
api->getModerators(tenantId, skip).then([](pplx::task<std::shared_ptr<GetModeratorsResponse>> t){
    auto response = t.get();
});
[inline-code-end]