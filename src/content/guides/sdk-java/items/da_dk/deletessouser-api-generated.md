## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | boolean | query | No |  |
| commentDeleteMode | string | query | No |  |

## Svar

Returnerer: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteSSOUserAPIResponse.java)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på deleteSSOUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Konfigurer API-nøgle-godkendelse: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Fjern kommentartegnet på følgende linje for at angive et præfiks for API-nøglen, f.eks. "Token" (standard er null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    Boolean deleteComments = true; // Boolean | 
    String commentDeleteMode = "commentDeleteMode_example"; // String | 
    try {
      DeleteSSOUserAPIResponse result = apiInstance.deleteSSOUser(tenantId, id)
            .deleteComments(deleteComments)
            .commentDeleteMode(commentDeleteMode)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteSSOUser");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]