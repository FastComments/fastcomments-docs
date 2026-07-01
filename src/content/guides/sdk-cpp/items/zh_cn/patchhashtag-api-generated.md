## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| tag | string | 是 |  |
| updateHashTagBody | UpdateHashTagBody | 是 |  |

## 响应

返回: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/UpdateHashTagResponse.h)

## 示例

[inline-code-attrs-start title = 'patchHashTag 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
utility::string_t tag = U("important");
auto body = std::make_shared<UpdateHashTagBody>();
body->name = U("important-updated");
body->description = boost::optional<utility::string_t>(U("Updated description"));
api->patchHashTag(tenantId, tag, *body)
   .then([](pplx::task<std::shared_ptr<UpdateHashTagResponse>> t){
       try{
           auto response = t.get();
           std::cout << "Updated tag ID: " << response->id << std::endl;
       }catch(const std::exception& e){
           std::cerr << "Patch failed: " << e.what() << std::endl;
       }
   });
[inline-code-end]