## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| options | const GetApiIdsOptions& | Da |  |

## Odgovor

Vraća: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/ModerationAPIGetCommentIdsResponse.h)

## Primer

[inline-code-attrs-start title = 'getApiIds Primer'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetApiIdsOptions options;
options.limit = boost::optional<int>(100);
options.cursor = boost::optional<utility::string_t>(U("next-page-token"));
api->getApiIds(tenantId, options).then([](pplx::task<std::shared_ptr<ModerationAPIGetCommentIdsResponse>> t){
    try{
        auto response = t.get();
        auto ids = response->commentIds;
    }catch(const std::exception&){
    }
});
[inline-code-end]