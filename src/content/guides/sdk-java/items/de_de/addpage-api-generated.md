## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Antwort

Gibt zurück: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AddPageAPIResponse.java)

## Beispiel

[inline-code-attrs-start title = 'addPage Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    
    // API-Schlüssel-Authentifizierung konfigurieren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Entkommentieren Sie die folgende Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standard ist null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateAPIPageData createAPIPageData = new CreateAPIPageData(); // CreateAPIPageData | 
    try {
      AddPageAPIResponse result = apiInstance.addPage(tenantId, createAPIPageData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#addPage");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---