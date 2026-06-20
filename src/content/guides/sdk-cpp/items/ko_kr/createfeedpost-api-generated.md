## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createFeedPostParams | CreateFeedPostParams | 예 |  |
| broadcastId | string | 아니요 |  |
| isLive | bool | 아니요 |  |
| doSpamCheck | bool | 아니요 |  |
| skipDupCheck | bool | 아니요 |  |

## 응답

반환: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostsResponse.h)

## 예제

[inline-code-attrs-start title = 'createFeedPost 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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