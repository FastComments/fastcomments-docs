## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| limit | number | query | Nein |  |
| skip | number | query | Nein |  |
| order | string | query | Nein |  |
| after | number | query | Nein |  |
| before | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogs200Response.java)

## Beispiel

[inline-code-attrs-start title = 'getAuditLogs Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Entkommentieren Sie die folgende Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standard: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double limit = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | 
    Double after = 3.4D; // Double | 
    Double before = 3.4D; // Double | 
    try {
      GetAuditLogs200Response result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]