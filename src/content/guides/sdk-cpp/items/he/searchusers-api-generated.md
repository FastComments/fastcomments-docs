## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| options | const SearchUsersOptions& | כן |  |

## Response

מחזיר: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/SearchUsersResult.h)

## Example

[inline-code-attrs-start title = 'דוגמת searchUsers'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t urlId = U("article-456");
SearchUsersOptions options;
options.query = U("john.doe@example.com");
options.page = boost::optional<int>(1);
options.pageSize = boost::optional<int>(20);
api->searchUsers(tenantId, urlId, options).then([](pplx::task<std::shared_ptr<SearchUsersResult>> task){
    try{
        auto result = task.get();
    }catch(const std::exception&){}
});
[inline-code-end]