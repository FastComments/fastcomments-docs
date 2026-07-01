---
列出租户的页面。供 FChat 桌面客户端填充其房间列表使用。  
每个页面的解析自定义配置中，需要将 `enableFChat` 设置为 true。  
需要 SSO 的页面将根据请求用户的组访问权限进行过滤。

## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | const GetPagesPublicOptions& | 是 |  |

## 响应

返回：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-cpp/blob/master/client/include/FastCommentsClient/model/client/include/FastCommentsClient/model/GetPublicPagesResponse.h)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'cpp'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
utility::string_t tenantId = U("my-tenant-123");
GetPagesPublicOptions options;
options.limit = boost::optional<int>(50);
options.cursor = boost::optional<utility::string_t>(U("cursor-token"));
api->getPagesPublic(tenantId, options).then([](pplx::task<std::shared_ptr<GetPublicPagesResponse>> task){
    try{
        auto response = task.get();
        // process response if needed
    }catch(const std::exception&){
        // handle error if needed
    }
});
[inline-code-end]

---