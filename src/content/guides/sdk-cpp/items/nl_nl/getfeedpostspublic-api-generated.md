req
tenantId
afterId

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| options | const GetFeedPostsPublicOptions& | Ja |  |

## Respons

Retourneert: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsPublic Voorbeeld'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto options = GetFeedPostsPublicOptions{};
options.limit = boost::optional<int>{20};
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){
    try{
        auto response = task.get();
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);
        // Gebruik processed indien nodig
    }catch(const std::exception&){
        // Fout afhandelen
    }
});
[inline-code-end]

---