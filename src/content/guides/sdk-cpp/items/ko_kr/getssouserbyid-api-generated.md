---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'getSSOUserById 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto ssoUserId = U("user-789");
api->getSSOUserById(tenantId, ssoUserId)
    .then([](std::shared_ptr<GetSSOUserByIdAPIResponse> resp) {
        boost::optional<utility::string_t> email;
        if (resp && resp->email) email = resp->email;
        if (email) {
            auto e = *email;
        }
    });
[inline-code-end]

---