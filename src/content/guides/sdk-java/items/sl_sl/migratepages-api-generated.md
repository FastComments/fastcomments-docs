Queues a batch job that moves comments, pages and related records from one urlId or domain to another.  
"urlId" migrations only match the exact urlId. "domain" migrations find/replace the value inside every urlId and url.  
Poll GET /api/v1/batch-jobs/{batchJobId}: once it is done it carries per‑migration results and an undoId.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |

## Odgovor

Vrne: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer migratePages'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    
    // Konfiguriraj avtorizacijo API ključa: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentiraj naslednjo vrstico, da nastaviš predpono za API ključ, npr. "Token" (privzeto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    MigrateAPIPagesData migrateAPIPagesData = new MigrateAPIPagesData(); // MigrateAPIPagesData | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.migratePages(tenantId, migrateAPIPagesData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Izjema pri klicu DefaultApi#migratePages
      System.err.println("Exception when calling DefaultApi#migratePages");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]