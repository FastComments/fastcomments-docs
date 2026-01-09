## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |
| order | string | query | Ne |  |
| after | number | query | Ne |  |
| before | number | query | Ne |  |

## Odgovor

Vraća: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogs200Response.java)

## Primjer

[inline-code-attrs-start title = 'getAuditLogs Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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
    
    // Konfiguriraj autorizaciju API ključa: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Otkomentirajte sljedeći redak kako biste postavili prefiks za API ključ, npr. "Token" (zadano je null)
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

---