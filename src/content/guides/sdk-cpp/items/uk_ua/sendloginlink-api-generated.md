## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## Відповідь

Повертає: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## Приклад

[inline-code-attrs-start title = 'sendLoginLink Приклад'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
api->sendLoginLink(
    U("my-tenant-123"),
    U("user@example.com"),
    boost::make_optional(U("https://myapp.com/auth/callback"))
).then([](std::shared_ptr<APIEmptyResponse> resp) {
});
[inline-code-end]