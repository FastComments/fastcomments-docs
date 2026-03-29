### FastComments API

FastComments 提供一个用于与许多资源交互的 API。您可以构建与我们平台的集成，甚至自己构建客户端！

在本文件中，您将找到 API 支持的所有资源及其请求和响应类型的文档。

对于企业客户，所有 API 访问都会被记录在审计日志中。

### 生成的 SDKs

FastComments 现在从我们的代码生成一个 [API Spec](https://fastcomments.com/js/swagger.json)（尚未完全完成，但已包含许多 API）。

我们现在也为常用语言提供 SDK：

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

### 认证

API 通过在请求中传递您的 [api key](https://fastcomments.com/auth/my-account/api-secret) 来进行认证，您可以将其放在 `X-API-KEY` 请求头或 `API_KEY` 查询参数中。您在调用 API 时还需要 `tenantId`。可以在与 api key 相同的页面上找到它。

### 安全注意事项

这些路由应当从 **服务器** 调用。__请勿__ 从浏览器调用它们。否则会泄露您的 API key —— 任何能够查看页面源代码的人都将获得对您帐户的完全访问权限！

#### 认证选项一 - 请求头

- 请求头: `X-API-KEY`
- 请求头: `X-TENANT-ID`

#### 认证选项二 - 查询参数

- 查询参数: `API_KEY`
- 查询参数: `tenantId`

### 读取您自己的写入

FastComments 提供双活（Active-Active）可用性。来自您数据中心的请求会被路由到离您最近的 [接入点](https://sophon.fastcomments.com/)。这是自动完成的，通常您可以观察到读写一致（read-your-write）语义。如果您想确保能读取到自己刚刚写入的数据，可以将请求固定到某个区域，通过使用该区域作为 API 主机来实现（不过对于大多数集成通常不需要）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

请注意，如果您这样做，可能需要定义一个回退方案，因为我们在过去曾弃用入口节点并为切换使用了新的名称。