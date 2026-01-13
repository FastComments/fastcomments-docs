## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| replaceTenantUserBody | ReplaceTenantUserBody | כן |  |
| updateComments | string | לא |  |

## תשובה

מחזיר: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-replaceTenantUser'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
ReplaceTenantUserBody replaceBody;
boost::optional<utility::string_t> updateComments = boost::optional<utility::string_t>(U("true"));
api->replaceTenantUser(tenantId, id, replaceBody, updateComments)
.then([](std::shared_ptr<FlagCommentPublic_200_response> resp){
    if (resp) {
        auto copy = std::make_shared<FlagCommentPublic_200_response>(*resp);
    }
});
[inline-code-end]

---