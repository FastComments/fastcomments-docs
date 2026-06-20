## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니오 | Used to determine whether the current page is subscribed. |
| pageSize | integer | query | 아니오 |  |
| afterId | string | query | 아니오 |  |
| includeContext | boolean | query | 아니오 |  |
| afterCreatedAt | integer | query | 아니오 |  |
| unreadOnly | boolean | query | 아니오 |  |
| dmOnly | boolean | query | 아니오 |  |
| noDm | boolean | query | 아니오 |  |
| includeTranslations | boolean | query | 아니오 |  |
| includeTenantNotifications | boolean | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMyNotificationsResponse.java)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 가져오기:
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
    String urlId = "urlId_example"; // String | 현재 페이지가 구독되어 있는지 여부를 판단하는 데 사용됩니다.
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