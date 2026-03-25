## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Svar

Returnerer: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateSubscriptionAPIResponse.java)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på updateSubscription'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer klasser:
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
    
    // Konfigurer API-nøgleautorisering: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Fjern kommentaren fra følgende linje for at sætte et præfiks for API-nøglen, f.eks. "Token" (standard er null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    UpdateAPIUserSubscriptionData updateAPIUserSubscriptionData = new UpdateAPIUserSubscriptionData(); // UpdateAPIUserSubscriptionData | 
    String userId = "userId_example"; // String | 
    try {
      UpdateSubscriptionAPIResponse result = apiInstance.updateSubscription(tenantId, id, updateAPIUserSubscriptionData)
            .userId(userId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#updateSubscription");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]