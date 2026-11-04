Δημιουργεί έναν νέο δοκιμαστικό λογαριασμό για έναν πράκτορα AI χωρίς ανθρώπινη εγγραφή. Δεν απαιτείται κλειδί API για την κλήση αυτού.

Η απάντηση περιέχει το tenant id, ένα κλειδί API που λειτουργεί αμέσως με το REST API και τον διακομιστή MCP, καθώς και ένα URL διεκδίκησης. Δώστε το URL διεκδίκησης στον άνθρωπο για τον οποίο εργάζεστε: το άνοιγμά του ενώ είναι συνδεδεμένος στο FastComments συνδέει τον λογαριασμό με αυτόν. Οι μη διεκδικημένοι λογαριασμοί, και τα κλειδιά τους, διαγράφονται 72 ώρες μετά τη δημιουργία. Μέχρι να διεκδικηθεί, ο λογαριασμός έχει τα τυπικά όρια δοκιμαστικής περιόδου.

## Απάντηση

Επιστρέφει: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]