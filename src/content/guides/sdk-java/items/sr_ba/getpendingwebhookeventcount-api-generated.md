## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| commentId | string | query | Не |  |
| externalId | string | query | Не |  |
| eventType | string | query | Не |  |
| type | string | query | Не |  |
| domain | string | query | Не |  |
| attemptCountGT | number | query | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPendingWebhookEventCount200Response.java)

## Примјер

[inline-code-attrs-start title = 'getPendingWebhookEventCount Примјер'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    
    // Постави ауторизацију API кључем: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Откоментирај следећу линију да подесиш префикс за API кључ, нпр. "Token" (подразумјевано је null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String externalId = "externalId_example"; // String | 
    String eventType = "eventType_example"; // String | 
    String type = "type_example"; // String | 
    String domain = "domain_example"; // String | 
    Double attemptCountGT = 3.4D; // Double | 
    try {
      GetPendingWebhookEventCount200Response result = apiInstance.getPendingWebhookEventCount(tenantId)
            .commentId(commentId)
            .externalId(externalId)
            .eventType(eventType)
            .type(type)
            .domain(domain)
            .attemptCountGT(attemptCountGT)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPendingWebhookEventCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---