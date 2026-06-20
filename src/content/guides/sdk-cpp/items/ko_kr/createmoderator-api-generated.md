## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createModeratorBody | CreateModeratorBody | 예 |  |

## 응답

반환: [`CreateModeratorResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModeratorResponse.h)

## 예제

[inline-code-attrs-start title = 'createModerator 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto bodyPtr = std::make_shared<CreateModeratorBody>();
bodyPtr->email = utility::string_t(U("moderator@mycompany.com"));
bodyPtr->role = utility::string_t(U("moderator"));
bodyPtr->displayName = boost::optional<utility::string_t>(U("Jane Moderator"));
bodyPtr->isActive = boost::optional<bool>(true);

api->createModerator(tenantId, *bodyPtr).then([](pplx::task<std::shared_ptr<CreateModeratorResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            utility::string_t newModeratorId = resp->id;
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---