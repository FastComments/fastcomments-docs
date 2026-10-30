Custom react counts for a page, optionally with the users who reacted.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "\"public\" παραλείπει χρήστες με ιδιωτικά προφίλ, \"include-private\" τα περιλαμβάνει." |

## Απάντηση

Επιστρέφει: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageReactsAPIResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPageReacts'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Απενεργοποιήστε το σχόλιο στην παρακάτω γραμμή για να ορίσετε πρόθεμα για το κλειδί API, π.χ. "Token" (προεπιλογή είναι null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | \"public\" παραλείπει χρήστες με ιδιωτικά προφίλ, \"include-private\" τα περιλαμβάνει.
    try {
      GetPageReactsAPIResponse result = apiInstance.getPageReacts(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Εξαίρεση κατά την κλήση του DefaultApi#getPageReacts");
      System.err.println("Κωδικός κατάστασης: " + e.getCode());
      System.err.println("Αιτία: " + e.getResponseBody());
      System.err.println("Κεφαλίδες απόκρισης: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---