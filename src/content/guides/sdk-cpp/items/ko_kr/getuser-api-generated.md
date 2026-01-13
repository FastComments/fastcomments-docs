## Parameters

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 응답

반환: [`GetUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUser_200_response.h)

## 예제

[inline-code-attrs-start title = 'getUser 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenant(U("my-tenant-123"));
boost::optional<utility::string_t> userId(U("user@example.com"));
api->getUser(tenant.value(), userId.value())
    .then([](pplx::task<std::shared_ptr<GetUser_200_response>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                auto result_copy = std::make_shared<GetUser_200_response>(*resp);
            } else {
                auto fallback = std::make_shared<GetUser_200_response>();
            }
        } catch (const std::exception &e) {
        }
    });
[inline-code-end]

---