## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createCommentParams | CreateCommentParams | はい |  |
| isLive | bool | いいえ |  |
| doSpamCheck | bool | いいえ |  |
| sendEmails | bool | いいえ |  |
| populateNotifications | bool | いいえ |  |

## レスポンス

戻り値: [`SaveComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SaveComment_200_response.h)

## 例

[inline-code-attrs-start title = 'saveComment の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto createParamsPtr = std::make_shared<CreateCommentParams>();
createParamsPtr->content = utility::string_t(U("This is a test comment posted via SDK"));
createParamsPtr->authorEmail = utility::string_t(U("user@example.com"));
createParamsPtr->threadId = utility::string_t(U("thread-456"));
boost::optional<bool> isLive(true);
boost::optional<bool> doSpamCheck(false);
boost::optional<bool> sendEmails(true);
boost::optional<bool> populateNotifications(false);
auto task = api->saveComment(tenantId, *createParamsPtr, isLive, doSpamCheck, sendEmails, populateNotifications)
.then([](std::shared_ptr<SaveComment_200_response> resp){
    if (resp) std::cout << "Comment saved successfully\n";
    else std::cout << "Failed to save comment\n";
});
task.wait();
[inline-code-end]

---