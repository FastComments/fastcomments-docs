## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetCommentsOptions& | Yes |  |

## Response

반환: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetCommentsResponse.h)

## Example

[inline-code-attrs-start title = 'getComments 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetCommentsOptions options;
options.page = 1;
options.limit = 50;
options.sort = U("newest");
options.authorEmail = boost::optional<utility::string_t>(U("user@example.com"));
api->getComments(tenantId, options).then([](pplx::task<std::shared_ptr<APIGetCommentsResponse>> task) {
    try {
        auto response = task.get();
        // 필요에 따라 응답 사용
    } catch (const std::exception& e) {
        // 오류 처리
    }
});
[inline-code-end]

---