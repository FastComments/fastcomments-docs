## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createCommentParams | CreateCommentParams | Да |  |
| isLive | bool | Не |  |
| doSpamCheck | bool | Не |  |
| sendEmails | bool | Не |  |
| populateNotifications | bool | Не |  |

## Одговор

Враћа: [`SaveComment_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SaveComment_200_response.h)

## Пример

[inline-code-attrs-start title = 'saveComment Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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