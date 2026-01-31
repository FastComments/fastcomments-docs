## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Response

Returns: [`GetVotes_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetVotes_200_response.h)

## Example

[inline-code-attrs-start title = 'getVotes Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
utility::string_t urlId = utility::conversions::to_string_t("https://www.example.com/articles/2026-new-features");
boost::optional<utility::string_t> pageCursor = boost::none;
api->getVotes(tenantId, urlId)
    .then([=](pplx::task<std::shared_ptr<GetVotes_200_response>> task) {
        try {
            auto resp = task.get();
            auto copy = std::make_shared<GetVotes_200_response>(*resp);
            (void)copy;
        } catch (...) {
        }
    }).wait();
[inline-code-end]
