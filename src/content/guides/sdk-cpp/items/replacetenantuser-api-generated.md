## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Response

Returns: [`FlagCommentPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentPublic_200_response.h)

## Example

[inline-code-attrs-start title = 'replaceTenantUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t id = U("user@example.com");
auto body = std::make_shared<ReplaceTenantUserBody>();
body->email = U("user@example.com");
body->displayName = U("Jane Example");
boost::optional<utility::string_t> updateComments = boost::optional<utility::string_t>(U("true"));
api->replaceTenantUser(tenantId, id, *body, updateComments)
.then([](pplx::task<std::shared_ptr<FlagCommentPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "Tenant user replaced successfully\n";
        } else {
            std::cout << "Replace returned empty response\n";
        }
    } catch (const std::exception& e) {
        std::cerr << "API error: " << e.what() << "\n";
    }
});
[inline-code-end]
