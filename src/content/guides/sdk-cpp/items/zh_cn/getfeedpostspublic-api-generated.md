req  
tenantId  
afterId  

## 参数  

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | const GetFeedPostsPublicOptions& | Yes |  |

## 响应  

返回: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## 示例  

[inline-code-attrs-start title = 'getFeedPostsPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
auto options = GetFeedPostsPublicOptions{};  
options.limit = boost::optional<int>{20};  
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};  

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){  
    try{  
        auto response = task.get();  
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);  
        // 根据需要使用 processed  
    }catch(const std::exception&){  
        // 处理错误  
    }  
});  
[inline-code-end]