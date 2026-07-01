req  
tenantId  
afterId  

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| options | const GetFeedPostsPublicOptions& | はい |  |

## レスポンス

返却: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/PublicFeedPostsResponse.h)

## 例

[inline-code-attrs-start title = 'getFeedPostsPublic の例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
auto options = GetFeedPostsPublicOptions{};  
options.limit = boost::optional<int>{20};  
options.before = boost::optional<utility::string_t>{U("2023-01-01T00:00:00Z")};  

api->getFeedPostsPublic(U("my-tenant-123"), options).then([](pplx::task<std::shared_ptr<PublicFeedPostsResponse>> task){  
    try{  
        auto response = task.get();  
        auto processed = std::make_shared<PublicFeedPostsResponse>(*response);  
        // 必要に応じて processed を使用  
    }catch(const std::exception&){  
        // エラーを処理  
    }  
});  
[inline-code-end]

---