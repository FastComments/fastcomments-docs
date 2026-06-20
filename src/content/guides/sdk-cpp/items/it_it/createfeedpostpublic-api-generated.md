---
## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| createFeedPostParams | CreateFeedPostParams | Sì |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`CreateFeedPostResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPostResponse.h)

## Esempio

[inline-code-attrs-start title = 'Esempio di createFeedPostPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto params = std::make_shared<CreateFeedPostParams>();
params->title = U("New feature release");
params->content = U("We launched the new comment moderation feature today.");
params->authorEmail = U("alice@example.com");
boost::optional<utility::string_t> broadcastId = boost::optional<utility::string_t>(U("broadcast-456"));
boost::optional<utility::string_t> sso = boost::optional<utility::string_t>(U("alice@example.com"));
api->createFeedPostPublic(tenantId, *params, broadcastId, sso)
.then([](pplx::task<std::shared_ptr<CreateFeedPostResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            utility::string_t postId = resp->postId;
        }
    } catch (const std::exception&) {
    }
});
[inline-code-end]

---