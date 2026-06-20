## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Όχι |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Όχι |  |

## Απάντηση

Επιστρέφει: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/BulkCreateHashTagsResponse.h)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα addHashTagsBulk'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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