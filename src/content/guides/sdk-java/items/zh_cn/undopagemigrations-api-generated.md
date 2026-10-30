Queues a batch job that reverses every migration from one migrate-pages job, newest first. Available for 90 days.  
将一个批处理作业排入队列，该作业会逆转来自单个 migrate-pages 作业的所有迁移，按最新的先处理。可用期限为 90 天。

Records changed since the migration are left alone, so a later edit is never overwritten.  
迁移后更改的记录保持不变，因此后续编辑不会被覆盖。

Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts.  
轮询 GET /api/v1/batch-jobs/{batchJobId} 以获取已恢复/已跳过的计数。

## Parameters

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| undoId | string | path | Yes |  |

## Response

返回：[`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Example

[inline-code-attrs-start title = 'undoPageMigrations 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // 配置 API 密钥授权: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 取消注释以下行以为 API 密钥设置前缀，例如 "Token"（默认 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String |
    String undoId = "undoId_example"; // String |
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.undoPageMigrations(tenantId, undoId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#undoPageMigrations");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---