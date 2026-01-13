---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createFeedPostParams | CreateFeedPostParams | 예 |  |
| broadcastId | string | 아니오 |  |
| isLive | bool | 아니오 |  |
| doSpamCheck | bool | 아니오 |  |
| skipDupCheck | bool | 아니오 |  |

## 응답

반환: [`CreateFeedPost_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPost_200_response.h)

## 예제

[inline-code-attrs-start title = 'createFeedPost 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
CreateFeedPostParams params;
params.title = utility::string_t(U("Weekly Product Update"));
params.content = utility::string_t(U("This week we shipped several improvements to the API."));
params.authorEmail = utility::string_t(U("jane.doe@example.com"));

boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-456"));
boost::optional<bool> isLive = boost::optional<bool>(true);
boost::optional<bool> doSpamCheck = boost::optional<bool>(false);
boost::optional<bool> skipDupCheck = boost::optional<bool>(false);

api->createFeedPost(tenantId, params, broadcastId, isLive, doSpamCheck, skipDupCheck)
.then([](pplx::task<std::shared_ptr<CreateFeedPost_200_response>> t){
    try {
        auto resp = t.get();
        auto copy = std::make_shared<CreateFeedPost_200_response>(*resp);
    } catch (...) {}
});
[inline-code-end]

---