## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 응답

반환: [`GetSSOUserByIdAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetSSOUserByIdAPIResponse.h)

## 예제

[inline-code-attrs-start title = 'getSSOUserById 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> optId = U("user-42@example.com");
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = optId.value_or(U("user-42@example.com"));
api->getSSOUserById(tenantId, id).then([](pplx::task<std::shared_ptr<GetSSOUserByIdAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto out = resp;
            (void)out;
        } else {
            auto fallback = std::make_shared<GetSSOUserByIdAPIResponse>();
            (void)fallback;
        }
    } catch (const std::exception& ex) {
        auto fallback = std::make_shared<GetSSOUserByIdAPIResponse>();
        (void)ex;
        (void)fallback;
    }
});
[inline-code-end]

---