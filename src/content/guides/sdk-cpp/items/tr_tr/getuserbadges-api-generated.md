## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetUserBadgesOptions& | Yes |  |

## Yanıt

Döndürür: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/APIGetUserBadgesResponse.h)

## Örnek

[inline-code-attrs-start title = 'getUserBadges Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetUserBadgesOptions opts;
opts.userId = boost::make_optional(U("user@example.com"));
opts.includeExpired = boost::make_optional(false);

api->getUserBadges(U("my-tenant-123"), opts)
   .then([](pplx::task<std::shared_ptr<APIGetUserBadgesResponse>> t) {
       try {
           auto response = t.get();
       } catch (const std::exception&) {
       }
   });
[inline-code-end]