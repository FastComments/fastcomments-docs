---
## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Hayır |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Hayır |  |

## Yanıt

Döndürür: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkCreateHashTagsResponse.h)

## Örnek

[inline-code-attrs-start title = 'addHashTagsBulk Örneği'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> tenantId(utility::conversions::to_string_t("my-tenant-123"));
auto bodyPtr = std::make_shared<BulkCreateHashTagsBody>();
boost::optional<BulkCreateHashTagsBody> bodyOpt(*bodyPtr);
api->addHashTagsBulk(tenantId, bodyOpt)
    .then([](pplx::task<std::shared_ptr<BulkCreateHashTagsResponse>> task) {
        try {
            auto resp = task.get();
            if (resp) {
                std::cout << "Bulk hashtags created successfully\n";
            } else {
                std::cout << "No response received\n";
            }
        } catch (const std::exception &e) {
            std::cerr << "API error: " << e.what() << '\n';
        }
    });
[inline-code-end]

---