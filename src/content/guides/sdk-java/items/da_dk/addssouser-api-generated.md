## Parametre

| Name | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AddSSOUserAPIResponse.java)

## Eksempel

[inline-code-attrs-start title = 'addSSOUser Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Konfigurer API-nøgleautorisation: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Fjern kommentar på følgende linje for at angive et præfiks for API-nøglen, f.eks. "Token" (standard er null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateAPISSOUserData createAPISSOUserData = new CreateAPISSOUserData(); // CreateAPISSOUserData | 
    try {
      AddSSOUserAPIResponse result = apiInstance.addSSOUser(tenantId, createAPISSOUserData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#addSSOUser");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---