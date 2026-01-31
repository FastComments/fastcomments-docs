## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createModeratorBody | CreateModeratorBody | Yes |  |

## Response

Returns: [`CreateModerator_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateModerator_200_response.h)

## Example

[inline-code-attrs-start title = 'createModerator Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateModeratorBody body;
body.email = utility::string_t(U("moderator@example.com"));
body.displayName = boost::optional<utility::string_t>(U("Site Moderator"));
body.roles = std::vector<utility::string_t>{ U("moderator") };
body.notes = boost::optional<utility::string_t>(U("Trusted for content moderation"));
auto resultHolder = std::make_shared<CreateModerator_200_response>();
api->createModerator(tenantId, body).then([resultHolder](std::shared_ptr<CreateModerator_200_response> resp){
    if (resp) {
        *resultHolder = *resp;
    }
    return resp;
});
[inline-code-end]
