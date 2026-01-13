## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| fromName | string | Da |  |

## Odgovor

Vrne: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Primer

[inline-code-attrs-start title = 'Primer sendInvite'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("comment-456");
utility::string_t fromName = U("moderator@example.com");
boost::optional<utility::string_t> inviteNote = boost::optional<utility::string_t>(U("Please review this flagged comment"));
auto task = api->sendInvite(tenantId, id, fromName)
    .then([inviteNote](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t) -> std::shared_ptr<FlagCommentPublic_200_response> {
        try {
            auto resp = t.get();
            auto wrapped = std::make_shared<FlagCommentPublic_200_response>(*resp);
            (void)inviteNote;
            return wrapped;
        } catch (...) {
            throw;
        }
    });
[inline-code-end]

---