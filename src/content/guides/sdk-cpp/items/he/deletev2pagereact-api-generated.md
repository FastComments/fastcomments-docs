## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| id | string | כן |  |

## תשובה

מחזיר: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/CreateV1PageReact.h)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteV2PageReact'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
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