### FastComments API

FastComments 提供用于与多个资源交互的 API。可与我们的平台构建集成，或构建您自己的客户端！

在本文档中，您将找到 API 支持的所有资源的文档及其请求和响应类型。

对于企业客户，所有 API 访问都会记录在审核日志中。

### 生成的 SDK

FastComments 现在从我们的代码生成了一个 [API 规范](https://fastcomments.com/js/swagger.json)（尚未完成，但包含许多 API）。

我们现在还为流行语言提供 SDK：

- [fastcomments-cpp](./guide-sdk-cpp.html)
- [fastcomments-go](./guide-sdk-go.html)
- [fastcomments-java](./guide-sdk-java.html)
- [fastcomments-sdk-js](./guide-sdk-javascript.html)
- [fastcomments-nim](./guide-sdk-nim.html)
- [fastcomments-php](guide-sdk-php.html)
- [fastcomments-php-sso](./guide-sdk-php-sso.html)
- [fastcomments-python](./guide-sdk-python.html)
- [fastcomments-ruby](./guide-sdk-ruby.html)
- [fastcomments-rust](./guide-sdk-rust.html)
- [fastcomments-swift](./guide-sdk-swift.html)

### 身份验证

API 的身份验证通过将您的 [API 密钥](https://fastcomments.com/auth/my-account/api-secret) 作为 `X-API-KEY` 请求头或 `API_KEY` 查询参数传递来完成。您在调用 API 时还需要 `tenantId`。可以从与您的 api key 相同的页面检索到它。

### 安全提示

这些路由应从 **服务器** 调用。 __切勿__ 从浏览器调用它们。这样会暴露您的 API 密钥——任何能查看页面源代码的人都将获得对您帐户的完全访问权限！

#### 身份验证选项一 - 请求头

- 请求头： `X-API-KEY`
- 请求头： `X-TENANT-ID`

#### 身份验证选项二 - 查询参数

- 查询参数： `API_KEY`
- 查询参数： `tenantId`

---