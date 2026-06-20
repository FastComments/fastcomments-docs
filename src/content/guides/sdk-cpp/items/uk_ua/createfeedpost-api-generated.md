## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createFeedPostParams | CreateFeedPostParams | Так |  |
| broadcastId | string | Ні |  |
| isLive | bool | Ні |  |
| doSpamCheck | bool | Ні |  |
| skipDupCheck | bool | Ні |  |

## Відповідь

Повертає: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostsResponse.h)

## Приклад

[inline-code-attrs-start title = 'Приклад createFeedPost'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::string_t("my-tenant-123");
auto paramsPtr = std::make_shared<CreateFeedPostParams>();
paramsPtr->content = utility::string_t("Deployment completed successfully. All services are operational.");
paramsPtr->authorEmail = utility::string_t("ops@company.com");
paramsPtr->authorName = utility::string_t("Deploy Bot");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(utility::string_t("broadcast-456"));
boost::optional<bool> isLive = boost::optional<bool>(true);
boost::optional<bool> doSpamCheck = boost::optional<bool>(true);
boost::optional<bool> skipDupCheck = boost::optional<bool>(false);
api->createFeedPost(tenantId, *paramsPtr, broadcastId, isLive, doSpamCheck, skipDupCheck)
    .then([](pplx::task<std::shared_ptr<CreateFeedPostsResponse>> t){
        try {
            auto resp = t.get();
            (void)resp;
        } catch (...) {}
    });
[inline-code-end]

---