## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUser_200_response.h)

## Example

[inline-code-attrs-start title = 'getUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t userId = U("user@example.com");
boost::optional<utility::string_t> acceptLanguage = boost::optional<utility::string_t>(U("en-US"));
pplx::task<std::shared_ptr<GetUser_200_response>> userTask = api->getUser(tenantId, userId)
    .then([](std::shared_ptr<GetUser_200_response> resp) -> std::shared_ptr<GetUser_200_response> {
        if (resp) return resp;
        return std::make_shared<GetUser_200_response>();
    });
userTask.wait();
std::shared_ptr<GetUser_200_response> userResult = userTask.get();
[inline-code-end]
