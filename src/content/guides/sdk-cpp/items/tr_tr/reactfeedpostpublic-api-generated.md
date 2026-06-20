## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| postId | string | Evet |  |
| reactBodyParams | ReactBodyParams | Evet |  |
| isUndo | bool | Hayır |  |
| broadcastId | string | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ReactFeedPostResponse.h)

## Örnek

[inline-code-attrs-start title = 'reactFeedPostPublic Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("feed-post-456");
ReactBodyParams reactBodyParams;
reactBodyParams.reaction = U("like");
boost::optional<bool> isUndo = false;
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-01"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->reactFeedPostPublic(tenantId, postId, reactBodyParams, isUndo, broadcastId, sso)
.then([](std::shared_ptr<ReactFeedPostResponse> resp) {
    if (resp) {
        auto resultCopy = std::make_shared<ReactFeedPostResponse>(*resp);
    }
});
[inline-code-end]