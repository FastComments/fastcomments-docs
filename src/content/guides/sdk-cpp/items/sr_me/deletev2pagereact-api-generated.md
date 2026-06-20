## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## Пример

[inline-code-attrs-start title = 'deleteV2PageReact Пример'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
boost::optional<utility::string_t> reason(U("cleanup"));
utility::string_t tenantId(U("my-tenant-123"));
utility::string_t urlId(U("blog/post-2024"));
utility::string_t id(U("react-0a1b2c3d"));
auto deleteTask = api->deleteV2PageReact(tenantId, urlId, id)
    .then([](std::shared_ptr<CreateV1PageReact> result){
        auto out = result ? result : std::make_shared<CreateV1PageReact>();
        return out;
    });
deleteTask.wait();
[inline-code-end]

---