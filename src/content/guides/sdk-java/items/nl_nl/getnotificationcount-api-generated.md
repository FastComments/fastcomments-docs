## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nee |  |
| urlId | string | query | Nee |  |
| fromCommentId | string | query | Nee |  |
| viewed | boolean | query | Nee |  |
| type | string | query | Nee |  |

## Antwoord

Geeft terug: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetNotificationCount200Response.java)

## Voorbeeld

[inline-code-attrs-start title = 'getNotificationCount Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Haal de volgende regel uit commentaar om een voorvoegsel voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String fromCommentId = "fromCommentId_example"; // String | 
    Boolean viewed = true; // Boolean | 
    String type = "type_example"; // String | 
    try {
      GetNotificationCount200Response result = apiInstance.getNotificationCount(tenantId)
            .userId(userId)
            .urlId(urlId)
            .fromCommentId(fromCommentId)
            .viewed(viewed)
            .type(type)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getNotificationCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]