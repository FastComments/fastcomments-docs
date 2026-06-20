## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| sure | string | 아니오 |  |

## 응답

반환: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIEmptyResponse.h)

## 예제

[inline-code-attrs-start title = 'deleteTenant 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("owner@example.com");
boost::optional<utility::string_t> sure = boost::optional<utility::string_t>(U("true"));
auto placeholder = std::make_shared<APIEmptyResponse>();
api->deleteTenant(tenantId, id, sure)
    .then([](std::shared_ptr<APIEmptyResponse> resp) {
        if (resp) {
            std::cout << "Tenant deleted successfully\n";
        } else {
            std::cout << "No response from deleteTenant\n";
        }
    })
    .wait();
[inline-code-end]

---