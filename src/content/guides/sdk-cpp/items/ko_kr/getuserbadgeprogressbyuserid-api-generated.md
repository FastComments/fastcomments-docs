## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |

## 응답

반환: [`GetUserBadgeProgressById_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetUserBadgeProgressById_200_response.h)

## 예제

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
boost::optional<utility::string_t> optUserId = utility::string_t(U("user@example.com"));
api->getUserBadgeProgressByUserId(tenantId, optUserId.value()).then(
    [](pplx::task<std::shared_ptr<GetUserBadgeProgressById_200_response>> t){
        try {
            auto resp = t.get();
            auto copy = std::make_shared<GetUserBadgeProgressById_200_response>(*resp);
        } catch (const std::exception&) {
        }
    }
);
[inline-code-end]

---