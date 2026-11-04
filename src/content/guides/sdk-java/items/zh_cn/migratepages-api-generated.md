将批量任务加入队列，把评论、页面及相关记录从一个 urlId 或域名迁移到另一个。
"urlId" 迁移仅匹配完全相同的 urlId。"domain" 迁移会在每个 urlId 和 url 中查找并替换该值。
轮询 GET /api/v1/batch-jobs/{batchJobId}：任务完成后会携带每次迁移的结果和一个 undoId。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 响应

返回：[`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## 示例

[inline-code-attrs-start title = 'migratePages 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类：
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
    // 取消以下行的注释以设置 API 密钥前缀，例如 "Token"（默认为 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    MigrateAPIPagesData migrateAPIPagesData = new MigrateAPIPagesData(); // MigrateAPIPagesData | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.migratePages(tenantId, migrateAPIPagesData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("调用 DefaultApi#migratePages 时发生异常");
      System.err.println("状态码: " + e.getCode());
      System.err.println("原因: " + e.getResponseBody());
      System.err.println("响应头: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]