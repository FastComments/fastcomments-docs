## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createFeedPostParams | CreateFeedPostParams | Evet |  |
| broadcastId | string | Hayır |  |
| isLive | bool | Hayır |  |
| doSpamCheck | bool | Hayır |  |
| skipDupCheck | bool | Hayır |  |

## Yanıt

Döndürür: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostsResponse.h)

## Örnek

[inline-code-attrs-start title = 'createFeedPost Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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