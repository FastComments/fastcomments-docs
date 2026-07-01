zahtjev
tenantId
urlId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | const GetCommentsPublicOptions& | Yes |  |

## Odgovor

Vraća: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## Primjer

[inline-code-attrs-start title = 'Primjer getCommentsPublic'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
auto tenantId = U("my-tenant-123");
auto urlId = U("article-456");
GetCommentsPublicOptions opts;
opts.pageSize = 20;
opts.includeDeleted = false;
api->getCommentsPublic(tenantId, urlId, opts).then([](std::shared_ptr<GetCommentsResponseWithPresence_PublicComment_> resp) {
    (void)resp;
});
[inline-code-end]