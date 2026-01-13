## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createModeratorBody | CreateModeratorBody | 예 |  |

## 응답

반환: [`CreateModerator_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModerator_200_response.h)

## 예제

[inline-code-attrs-start title = 'createModerator 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateModeratorBody createModeratorBody;
createModeratorBody.email = U("moderator@example.com");
createModeratorBody.displayName = U("Support Moderator");
createModeratorBody.roles = std::vector<utility::string_t>{ U("moderator"), U("support") };
createModeratorBody.notes = boost::optional<utility::string_t>(U("Temporary moderator for Q1"));
api->createModerator(tenantId, createModeratorBody)
.then([](pplx::task<std::shared_ptr<CreateModerator_200_response>> t)
{
    try
    {
        auto resp = t.get();
        auto respCopy = std::make_shared<CreateModerator_200_response>(*resp);
        (void)respCopy;
    }
    catch (const std::exception&)
    {
    }
});
[inline-code-end]

---