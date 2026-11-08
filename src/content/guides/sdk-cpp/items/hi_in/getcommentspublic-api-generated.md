req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| options | const GetCommentsPublicOptions& | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsResponseWithPresence_PublicComment_`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/GetCommentsResponseWithPresence_PublicComment_.h)

## उदाहरण

[inline-code-attrs-start title = 'getCommentsPublic उदाहरण'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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