## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 | 要读取审计日志的租户。 |
| limit | integer | query | 否 | 返回的日志数量。最大 10000，默认 1000。 |
| skip | integer | query | 否 | 要跳过的日志数量，用于分页。 |
| order | string | query | 否 | 事件时间的排序方向。默认 DESC（最新的在前）。 |
| after | integer | query | 否 |  |
| before | integer | query | 否 |  |
| username | string | query | 否 | 仅返回由此用户名执行的事件。 |
| ip | string | query | 否 | 仅返回来自此 IP 地址的事件。 |
| crudType | string | query | 否 | 仅返回此类型的事件：c、r、u、d 或 login。 |
| resourceName | string | query | 否 | 仅返回此资源的事件，例如 User 或 Moderator。 |
| targetId | string | query | 否 | 仅返回受影响对象具有此 ID 的事件。 |
| target | string | query | 否 | 对受影响对象标签进行不区分大小写的子字符串匹配，例如用户名或电子邮件。 |
| includeManagedTenants | boolean | query | 否 | 还返回此租户管理的租户的事件。 |

## 响应

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## 示例

[inline-code-attrs-start title = 'getAuditLogs 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // 配置 API 密钥授权：api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 取消注释以下行以为 API 密钥设置前缀，例如 "Token"（默认 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 要读取审计日志的租户。
    Integer limit = 56; // Integer | 返回的日志数量。最大 10000，默认 1000。
    Integer skip = 56; // Integer | 要跳过的日志数量，用于分页。
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | 事件时间的排序方向。默认 DESC（最新的在前）。
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | 仅返回由此用户名执行的事件。
    String ip = "ip_example"; // String | 仅返回来自此 IP 地址的事件。
    String crudType = "crudType_example"; // String | 仅返回此类型的事件：c、r、u、d 或 login。
    String resourceName = "resourceName_example"; // String | 仅返回此资源的事件，例如 User 或 Moderator。
    String targetId = "targetId_example"; // String | 仅返回受影响对象具有此 ID 的事件。
    String target = "target_example"; // String | 对受影响对象标签进行不区分大小写的子字符串匹配，例如用户名或电子邮件。
    Boolean includeManagedTenants = true; // Boolean | 还返回此租户管理的租户的事件。
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]