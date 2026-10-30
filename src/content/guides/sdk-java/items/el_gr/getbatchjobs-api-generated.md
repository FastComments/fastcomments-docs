The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| type | string | query | Όχι | Μόνο εργασίες αυτού του τύπου, π.χ. migrate-pages. |
| status | string | query | Όχι | Μόνο εργασίες σε αυτή την κατάσταση: requested, running, success ή failed. |
| skip | integer | query | Όχι | Πόσες εργασίες να παραληφθούν, για σελιδοποίηση. |

## Απόκριση

Επιστρέφει: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getBatchJobs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Αφαιρέστε το σχόλιο από την παρακάτω γραμμή για να ορίσετε πρόθεμα για το κλειδί API, π.χ. "Token" (προεπιλογή null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | Μόνο εργασίες αυτού του τύπου, π.χ. migrate-pages.
    String status = "status_example"; // String | Μόνο εργασίες σε αυτή την κατάσταση: requested, running, success ή failed.
    Integer skip = 56; // Integer | Πόσες εργασίες να παραληφθούν, για σελιδοποίηση.
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getBatchJobs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]