## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | いいえ | 使用して現在のページが購読されているかどうかを判定します。 |
| pageSize | integer | query | いいえ |  |
| afterId | string | query | いいえ |  |
| includeContext | boolean | query | いいえ |  |
| afterCreatedAt | integer | query | いいえ |  |
| unreadOnly | boolean | query | いいえ |  |
| dmOnly | boolean | query | いいえ |  |
| noDm | boolean | query | いいえ |  |
| includeTranslations | boolean | query | いいえ |  |
| includeTenantNotifications | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

返却値: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMyNotificationsResponse.java)

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
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
    String urlId = "urlId_example"; // String | 現在のページが購読されているかどうかを判定するために使用されます。
    Integer pageSize = 56; // Integer | 
    String afterId = "afterId_example"; // String | 
    Boolean includeContext = true; // Boolean | 
    Long afterCreatedAt = 56L; // Long | 
    Boolean unreadOnly = true; // Boolean | 
    Boolean dmOnly = true; // Boolean | 
    Boolean noDm = true; // Boolean | 
    Boolean includeTranslations = true; // Boolean | 
    Boolean includeTenantNotifications = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      GetMyNotificationsResponse result = apiInstance.getUserNotifications(tenantId)
            .urlId(urlId)
            .pageSize(pageSize)
            .afterId(afterId)
            .includeContext(includeContext)
            .afterCreatedAt(afterCreatedAt)
            .unreadOnly(unreadOnly)
            .dmOnly(dmOnly)
            .noDm(noDm)
            .includeTranslations(includeTranslations)
            .includeTenantNotifications(includeTenantNotifications)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUserNotifications");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]