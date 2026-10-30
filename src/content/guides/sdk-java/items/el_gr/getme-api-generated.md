---
Αναγνωρίζει το διαπιστευτήριο σε χρήση: τον ενοικιαστή στο οποίο ανήκει και, για διακριτικά OAuth, τον χρήστη που το εξουσιοδότησε.
Οι ενσωματώσεις το χρησιμοποιούν για να δοκιμάσουν μια σύνδεση και να την ετικετοποιήσουν.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Ναι |  |

## Response

Επιστρέφει: [`GetMeResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMeResponse.java)

## Example

[inline-code-attrs-start title = 'Παράδειγμα getMe'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    try {
      GetMeResponse result = apiInstance.getMe(tenantId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getMe");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---