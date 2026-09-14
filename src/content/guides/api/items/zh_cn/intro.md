### FastComments API

FastComments 提供了一个用于与众多资源交互的 API。您可以构建与我们平台的集成，甚至自行编写客户端！

在本文档中，您将找到 API 支持的所有资源及其请求和响应类型的文档。

对于企业客户，所有 API 访问都会记录在审计日志中。

### 生成的 SDK

FastComments 现在可以从我们的代码生成 [API 规范](https://fastcomments.com/js/swagger.json)（尚未完整，但已包含许多 API）。

我们还提供了流行语言的 SDK：

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

API 通过将您的 [api key](https://fastcomments.com/auth/my-account/api-secret) 作为 `X-API-KEY` 请求头或 `API_KEY` 查询参数传递进行身份验证。您还需要提供 `tenantId` 来进行 API 调用。该信息可在与 api key 同一页面获取。

### 安全说明

这些路由应当从 **服务器** 调用。__请勿__ 从浏览器调用。这样会泄露您的 API 密钥——任何能够查看页面源代码的人都将获得对您账户的完整访问权限！

#### 身份验证选项一 - 请求头

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### 身份验证选项二 - 查询参数

- Query Param: `API_KEY`
- Query Param: `tenantId`

#### 身份验证选项三 - OAuth Bearer Token

- Header: `Authorization: Bearer fcat_...`

Zapier 等第三方应用以及 [MCP server](https://docs.fastcomments.com/guide-llm-kit.html) 的客户端通过 OAuth 获取令牌，而不是使用 API 密钥。该令牌可用于此处的所有端点。租户信息由令牌隐含，因此 `tenantId` 为可选项，但若提供必须与令牌匹配。`GET` 请求需要 `read` 范围，其他方法需要 `write` 范围。完整流程（包括客户端注册、PKCE、刷新和撤销）已在 [OAuth Authorization](#oauth) 中记录。发现过程从 `https://fastcomments.com/.well-known/oauth-authorization-server` 开始。

### 读取自己的写入

FastComments 提供主动-主动可用性。来自您数据中心的请求会被路由到离您最近的 [接入点](https://sophon.fastcomments.com/)。这一步是自动完成的，通常您可以观察到读写一致性语义。如果您想确保读取到自己的写入，可以通过将请求固定到特定区域的 API 主机来实现（不过大多数集成通常不需要这样做）：

- gdc-oregon.fastcomments.com
- gdc-virginia.fastcomments.com
- gdc-singapore.fastcomments.com
- gdc-falkenstein2.fastcomments.com
- gdc-sao-paulo.fastcomments.com
- eudc-helsinki2.fastcomments.com
- eudc-limburg.fastcomments.com
- eudc-france.fastcomments.com

请注意，如果您这样做，可能需要定义回退方案，因为我们过去已弃用某些入口节点，并在切换时使用了新名称。