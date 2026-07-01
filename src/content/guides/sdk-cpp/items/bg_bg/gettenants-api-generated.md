## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | const GetTenantsOptions& | Да |  |

## Отговор

Връща: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetTenantsResponse.h)

## Пример

[inline-code-attrs-start title = 'Пример getTenants'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GetTenantsOptions options;
options.includeDeleted = boost::make_optional(false);
options.searchTerm = boost::make_optional(utility::string_t(U("enterprise")));

api->getTenants(utility::string_t(U("my-tenant-123")), options)
    .then([](std::shared_ptr<GetTenantsResponse> response) {
    })
    .then([](pplx::task<void> t){ try{ t.get(); }catch(...){} });
[inline-code-end]