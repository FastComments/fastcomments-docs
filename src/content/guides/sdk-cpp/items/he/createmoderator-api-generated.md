## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| createModeratorBody | CreateModeratorBody | כן |  |

## תגובה

מחזיר: [`CreateModerator_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModerator_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createModerator'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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