目前在線的頁面觀眾：目前其 websocket 會話已訂閱該頁面的人員。
返回 anonCount + totalCount（房間範圍的訂閱者，包括我們不列舉的匿名觀眾）。

## 參數

| 名稱 | 類型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別碼（伺服器端清理）。 |
| afterName | string | query | 否 | 游標：從先前的回應中傳入 nextAfterName。 |
| afterUserId | string | query | 否 | 游標平手決勝：從先前的回應中傳入 nextAfterUserId。當設定 afterName 時需要此參數，以免同名導致條目遺失。 |

## 回應

回傳: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## 範例

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別：
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 頁面 URL 識別碼（伺服器端清理）。
    String afterName = "afterName_example"; // String | 游標：從先前的回應中傳入 nextAfterName。
    String afterUserId = "afterUserId_example"; // String | 游標平手決勝：從先前的回應中傳入 nextAfterUserId。當設定 afterName 時需要此參數，以免同名導致條目遺失。
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]