## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 | 要讀取審核日誌的租戶。 |
| limit | integer | query | 否 | 要返回的日誌數量。最大 10000，預設為 1000。 |
| skip | integer | query | 否 | 要跳過的日誌數量，用於分頁。 |
| order | string | query | 否 | 事件時間的排序方向。預設為 DESC（最新的在前）。 |
| after | integer | query | 否 |  |
| before | integer | query | 否 |  |
| username | string | query | 否 | 僅返回此使用者名稱執行的事件。 |
| ip | string | query | 否 | 僅返回來自此 IP 位址的事件。 |
| crudType | string | query | 否 | 僅返回此類型的事件：c、r、u、d，或 login。 |
| resourceName | string | query | 否 | 僅返回此資源的事件，例如 User 或 Moderator。 |
| targetId | string | query | 否 | 僅返回受影響物件具有此 ID 的事件。 |
| target | string | query | 否 | 對受影響物件的標籤（例如使用者名稱或電子郵件）進行不區分大小寫的子字串匹配。 |
| includeManagedTenants | boolean | query | 否 | 同時返回此租戶所管理的租戶的事件。 |

## 回應

回傳：[`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## 範例

[inline-code-attrs-start title = 'getAuditLogs 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別:
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
    
    // 設定 API 金鑰授權：api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 取消註解以下行以設定 API 金鑰的前綴，例如 "Token"（預設為 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 要讀取審核日誌的租戶。
    Integer limit = 56; // Integer | 要返回的日誌數量。最大 10000，預設為 1000。
    Integer skip = 56; // Integer | 要跳過的日誌數量，用於分頁。
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | 事件時間的排序方向。預設為 DESC（最新的在前）。
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | 僅返回此使用者名稱執行的事件。
    String ip = "ip_example"; // String | 僅返回來自此 IP 位址的事件。
    String crudType = "crudType_example"; // String | 僅返回此類型的事件：c、r、u、d，或 login。
    String resourceName = "resourceName_example"; // String | 僅返回此資源的事件，例如 User 或 Moderator。
    String targetId = "targetId_example"; // String | 僅返回受影響物件具有此 ID 的事件。
    String target = "target_example"; // String | 對受影響物件的標籤（例如使用者名稱或電子郵件）進行不區分大小寫的子字串匹配。
    Boolean includeManagedTenants = true; // Boolean | 同時返回此租戶所管理的租戶的事件。
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