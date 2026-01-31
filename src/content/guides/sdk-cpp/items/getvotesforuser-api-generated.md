## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`GetVotesForUser_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotesForUser_200_response.h)

## Example

[inline-code-attrs-start title = 'getVotesForUser Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
boost::optional<utility::string_t> userId(U("user@example.com"));
boost::optional<utility::string_t> anonUserId;

api->getVotesForUser(tenantId, urlId, userId, anonUserId)
.then([](pplx::task<std::shared_ptr<GetVotesForUser_200_response>> task){
    try {
        auto resp = task.get();
        if (!resp) return;
        auto copy = std::make_shared<GetVotesForUser_200_response>(*resp);
        std::cout << "Retrieved votes for user\n";
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << '\n';
    }
});
[inline-code-end]
