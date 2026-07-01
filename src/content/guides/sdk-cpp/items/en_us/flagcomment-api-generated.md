## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| options | const FlagCommentOptions& | Yes |  |

## Response

Returns: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/FlagCommentResponse.h)

## Example

[inline-code-attrs-start title = 'flagComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto opts = std::make_shared<FlagCommentOptions>();
opts->reason = utility::conversions::to_string_t("spam");
opts->note = boost::optional<utility::string_t>(utility::conversions::to_string_t("User posted duplicate links"));

api->flagComment(utility::conversions::to_string_t("my-tenant-123"),
                 utility::conversions::to_string_t("comment-456"),
                 *opts)
    .then([](pplx::task<std::shared_ptr<FlagCommentResponse>> t) {
        auto resp = t.get();
    });
[inline-code-end]
