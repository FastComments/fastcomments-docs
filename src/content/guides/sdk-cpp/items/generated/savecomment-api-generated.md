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
utility::string_t tenantId = U("my-tenant-123");
CreateCommentParams createParams;
createParams.body = U("This is a test comment from the C++ SDK");
createParams.authorEmail = U("user@example.com");
createParams.threadId = U("thread-456");
boost::optional<bool> isLive(true);
boost::optional<bool> doSpamCheck(false);
boost::optional<bool> sendEmails(true);
boost::optional<bool> populateNotifications(true);
api->saveComment(tenantId, createParams, isLive, doSpamCheck, sendEmails, populateNotifications)
.then([](pplx::task<std::shared_ptr<SaveComment_200_response>> task){
    try {
        auto resp = task.get();
        if (resp) {
            auto copy = std::make_shared<SaveComment_200_response>(*resp);
        }
    } catch (...) {
    }
});
[inline-code-end]
