## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | 任意 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 任意 |  |

## レスポンス

戻り値: [`AddHashTagsBulk_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddHashTagsBulk_200_response.h)

## 例

[inline-code-attrs-start title = 'addHashTagsBulk の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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