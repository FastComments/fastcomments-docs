## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postId | string | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`DeleteFeedPostPublic_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/DeleteFeedPostPublic_200_response.h)

## 예제

[inline-code-attrs-start title = 'deleteFeedPostPublic 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t postId = U("post-456");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-789"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("user@example.com"));
api->deleteFeedPostPublic(tenantId, postId, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<DeleteFeedPostPublic_200_response>> t){
    try {
        auto resp = t.get();
        if (!resp) resp = std::make_shared<DeleteFeedPostPublic_200_response>();
        (void)resp;
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---