## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 示例

[inline-code-attrs-start title = 'putReopenThread 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->putReopenThread(utility::string_t(U("my-tenant-123")), utility::string_t(U("thread-456")), boost::make_optional<utility::string_t>(U("user@example.com")))
    .then([](std::shared_ptr<APIEmptyResponse> result){
        std::cout << "Thread reopened" << std::endl;
    });
[inline-code-end]

---