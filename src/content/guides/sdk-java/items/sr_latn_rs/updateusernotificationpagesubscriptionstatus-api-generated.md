Omogući ili onemogući obaveštenja za stranicu. Kada su korisnici pretplaćeni na stranicu, obaveštenja se kreiraju
for new root comments, and also

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| url | string | query | Yes |  |
| pageTitle | string | query | Yes |  |
| subscribedOrUnsubscribed | string | path | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationPageSubscriptionStatusResponse.java)

## Primer

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvoz klasa:
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
    String urlId = "urlId_example"; // String | 
    String url = "url_example"; // String | 
    String pageTitle = "pageTitle_example"; // String | 
    String subscribedOrUnsubscribed = "subscribe"; // String | 
    String sso = "sso_example"; // String | 
    try {
      UpdateUserNotificationPageSubscriptionStatusResponse result = apiInstance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#updateUserNotificationPageSubscriptionStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]