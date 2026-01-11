## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createAPIPageData | CreateAPIPageData | Yes |  |

## Response

Returns: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddPageAPIResponse.h)

## Example

[inline-code-attrs-start title = 'addPage Example'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
auto createReq = std::make_shared<CreateAPIPageData>();
createReq->url = U("https://docs.example.com/getting-started");
createReq->title = U("Getting Started Guide");
createReq->authorEmail = U("doc-owner@example.com");
createReq->isPublished = boost::optional<bool>(true);
createReq->summary = boost::optional<utility::string_t>(U("Introductory setup and usage"));
api->addPage(tenantId, *createReq).then([](pplx::task<std::shared_ptr<AddPageAPIResponse>> task){
    try {
        auto resp = task.get();
        if (resp) {
            utility::string_t pageId = resp->pageId;
            utility::string_t status = resp->status;
            (void)pageId; (void)status;
        }
    } catch (const std::exception& e) {
        (void)e;
    }
});
[inline-code-end]
