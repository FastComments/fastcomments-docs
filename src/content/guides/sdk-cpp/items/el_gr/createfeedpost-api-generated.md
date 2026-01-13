## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| createFeedPostParams | CreateFeedPostParams | Ναι |  |
| broadcastId | string | Όχι |  |
| isLive | bool | Όχι |  |
| doSpamCheck | bool | Όχι |  |
| skipDupCheck | bool | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateFeedPost_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateFeedPost_200_response.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createFeedPost'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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