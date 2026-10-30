Queues a batch job that reverses every migration from one migrate‑pages job, newest first. Available for 90 days.  
Records changed since the migration are left alone, so a later edit is never overwritten.  
Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| undoId | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'undoPageMigrations Παράδειγμα'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Εισαγωγή κλάσεων:
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
    
    // Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε πρόθεμα για το κλειδί API, π.χ. "Token" (προεπιλογή είναι null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String undoId = "undoId_example"; // String | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.undoPageMigrations(tenantId, undoId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#undoPageMigrations");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]