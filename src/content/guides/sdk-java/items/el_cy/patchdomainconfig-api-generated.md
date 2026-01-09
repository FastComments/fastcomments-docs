## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Απόκριση

Επιστρέφει: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetDomainConfig200Response.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patchDomainConfig'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Ρύθμιση εξουσιοδότησης με κλειδί API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Αποσχολίασε την παρακάτω γραμμή για να ορίσεις πρόθεμα για το κλειδί API, π.χ. "Token" (προεπιλεγμένο null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String domainToUpdate = "domainToUpdate_example"; // String | 
    PatchDomainConfigParams patchDomainConfigParams = new PatchDomainConfigParams(); // PatchDomainConfigParams | 
    try {
      GetDomainConfig200Response result = apiInstance.patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#patchDomainConfig");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]