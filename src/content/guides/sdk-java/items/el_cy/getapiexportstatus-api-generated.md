## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Απόκριση

Επιστρέφει: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationExportStatusResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'getApiExportStatus Παράδειγμα'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Εισαγωγή κλάσεων:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String batchJobId = "batchJobId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationExportStatusResponse result = apiInstance.getApiExportStatus(tenantId)
            .batchJobId(batchJobId)
            .sno(sno)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Εξαίρεση κατά την κλήση του ModerationApi#getApiExportStatus");
      System.err.println("Κωδικός κατάστασης: " + e.getCode());
      System.err.println("Αιτία: " + e.getResponseBody());
      System.err.println("Κεφαλίδες ανταπόκρισης: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]