過去在該頁面發表評論但目前未在線的使用者。依 displayName 排序。
在用盡 /users/online 後使用此項來呈現「成員」區塊。
針對 commenterName 的游標分頁：伺服器會在部分索引 {tenantId, urlId, commenterName} 上從 afterName 向前走，使用 $gt，無 $skip 成本。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別符（伺服器端清理）。 |
| afterName | string | query | No | 游標：傳入先前回應的 nextAfterName。 |
| afterUserId | string | query | No | 游標平手解決：傳入先前回應的 nextAfterUserId。當設定 afterName 時為必要，以免同名導致條目被遺漏。 |

## 回應

回傳：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 頁面 URL 識別符（伺服器端清理）。
    String afterName = "afterName_example"; // String | 游標：傳入先前回應的 nextAfterName。
    String afterUserId = "afterUserId_example"; // String | 游標平手解決：傳入先前回應的 nextAfterUserId。當設定 afterName 時為必要，以免同名導致條目被遺漏。
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]