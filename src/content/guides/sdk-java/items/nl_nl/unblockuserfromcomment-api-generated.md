## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Response

Retourneert: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UnBlockCommentPublic200Response.java)

## Voorbeeld

[inline-code-attrs-start title = 'unBlockUserFromComment Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    
    // Configureer API-sleutelautorisatie: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Haal de volgende regel uit commentaar om een voorvoegsel voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    UnBlockFromCommentParams unBlockFromCommentParams = new UnBlockFromCommentParams(); // UnBlockFromCommentParams | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    try {
      UnBlockCommentPublic200Response result = apiInstance.unBlockUserFromComment(tenantId, id, unBlockFromCommentParams)
            .userId(userId)
            .anonUserId(anonUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#unBlockUserFromComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]