## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Не |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | Не |  |

## Одговор

Враћа: [`AddHashTagsBulk_200_response`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/AddHashTagsBulk_200_response.h)

## Пример

[inline-code-attrs-start title = 'Пример addHashTagsBulk'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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