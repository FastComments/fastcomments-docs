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
auto createCommentParams = std::make_shared<CreateCommentParams>();
createCommentParams->content = U("This is a test comment submitted via API");
createCommentParams->authorEmail = U("user@example.com");
createCommentParams->threadId = U("thread-456");
boost::optional<bool> isLiveOpt(true);
boost::optional<bool> doSpamCheckOpt(false);
boost::optional<bool> sendEmailsOpt(true);
boost::optional<bool> populateNotificationsOpt(false);
api->saveComment(tenantId, *createCommentParams, isLiveOpt, doSpamCheckOpt, sendEmailsOpt, populateNotificationsOpt)
.then([](std::shared_ptr<SaveComment_200_response> resp){
    return pplx::task_from_result(resp);
})
.wait();
[inline-code-end]
