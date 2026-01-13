## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createCommentParams | CreateCommentParams | Yes |  |
| isLive | bool | No |  |
| doSpamCheck | bool | No |  |
| sendEmails | bool | No |  |
| populateNotifications | bool | No |  |

## Response

Returns: [`SaveComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SaveComment_200_response.h)

## Example

[inline-code-attrs-start title = 'saveComment Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto params = std::make_shared<CreateCommentParams>();
params->content = utility::conversions::to_string_t("This is a test comment on the product page");
params->authorEmail = utility::conversions::to_string_t("jane.doe@example.com");
params->authorName = utility::conversions::to_string_t("Jane Doe");
params->threadId = utility::conversions::to_string_t("product-123-thread");

boost::optional<bool> isLive(true);
boost::optional<bool> doSpamCheck(true);
boost::optional<bool> sendEmails(false);
boost::optional<bool> populateNotifications(true);

api->saveComment(
    utility::conversions::to_string_t("my-tenant-123"),
    *params,
    isLive,
    doSpamCheck,
    sendEmails,
    populateNotifications
).then([](pplx::task<std::shared_ptr<SaveComment_200_response>> t) {
    try {
        auto resp = t.get();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]
