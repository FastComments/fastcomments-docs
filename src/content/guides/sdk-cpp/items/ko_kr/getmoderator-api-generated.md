## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## 응답

반환: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetModeratorResponse.h)

## 예제

[inline-code-attrs-start title = 'getModerator 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenant = utility::string_t(U("my-tenant-123"));
boost::optional<utility::string_t> moderatorId = utility::string_t(U("moderator-456"));
api->getModerator(tenant.value(), moderatorId.value())
    .then([](pplx::task<std::shared_ptr<GetModeratorResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) return std::make_shared<GetModeratorResponse>(*resp);
            return std::shared_ptr<GetModeratorResponse>();
        } catch (...) {
            return std::shared_ptr<GetModeratorResponse>();
        }
    })
    .then([](std::shared_ptr<GetModeratorResponse> result) {
        if (result) {
            /* use result */
        }
    });
[inline-code-end]

---