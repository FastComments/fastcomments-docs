## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не | Използва се за определяне дали текущата страница е абонирана. |
| pageSize | integer | query | Не |  |
| afterId | string | query | Не |  |
| includeContext | boolean | query | Не |  |
| afterCreatedAt | integer | query | Не |  |
| unreadOnly | boolean | query | Не |  |
| dmOnly | boolean | query | Не |  |
| noDm | boolean | query | Не |  |
| includeTranslations | boolean | query | Не |  |
| includeTenantNotifications | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMyNotificationsResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getUserNotifications'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортирай класове:
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
    String urlId = "urlId_example"; // String | Използва се за определяне дали текущата страница е абонирана.
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

---