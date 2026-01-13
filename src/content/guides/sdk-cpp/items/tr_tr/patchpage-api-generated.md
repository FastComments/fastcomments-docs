## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |
| updateAPIPageData | UpdateAPIPageData | Evet |  |

## Yanıt

Döndürür: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PatchPageAPIResponse.h)

## Örnek

[inline-code-attrs-start title = 'patchPage Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t pageId = U("page-9876");
auto updatePtr = std::make_shared<UpdateAPIPageData>();
updatePtr->title = utility::string_t(U("About Our Team"));
updatePtr->slug = utility::string_t(U("about-our-team"));
updatePtr->enabled = boost::optional<bool>(true);
updatePtr->description = boost::optional<utility::string_t>(U("Updated team overview and contact information"));
api->patchPage(tenantId, pageId, *updatePtr)
.then([](pplx::task<std::shared_ptr<PatchPageAPIResponse>> t){
    try {
        auto resp = t.get();
        if (resp) {
            std::cout << "Patch successful, page id: " << resp->id << std::endl;
        }
    } catch (const std::exception &e) {
        std::cerr << "Patch failed: " << e.what() << std::endl;
    }
});
[inline-code-end]

---