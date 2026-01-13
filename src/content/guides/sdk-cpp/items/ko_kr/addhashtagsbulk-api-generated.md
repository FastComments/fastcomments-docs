---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 아니요 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 아니요 |  |

## 응답

반환: [`AddHashTagsBulk_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddHashTagsBulk_200_response.h)

## 예제

[inline-code-attrs-start title = 'addHashTagsBulk 예제'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = utility::conversions::to_string_t("my-tenant-123");
auto bodyPtr = std::make_shared<BulkCreateHashTagsBody>();
boost::optional<utility::string_t> optTenant(tenantId);
boost::optional<BulkCreateHashTagsBody> optBody(*bodyPtr);
api->addHashTagsBulk(optTenant, optBody).then([](pplx::task<std::shared_ptr<AddHashTagsBulk_200_response>> t){
    try {
        auto resp = t.get();
        if (resp) std::cout << "Bulk hashtags added for tenant\n";
    } catch (const std::exception &e) {
        std::cerr << "addHashTagsBulk error: " << e.what() << '\n';
    }
});
[inline-code-end]

---