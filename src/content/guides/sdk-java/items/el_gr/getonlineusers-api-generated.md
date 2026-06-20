Τρέχοντες συνδεδεμένοι θεατές μιας σελίδας: άτομα των οποίων η websocket συνεδρία είναι εγγεγραμμένη στη σελίδα αυτή τώρα.
Επιστρέφει anonCount + totalCount (συνδρομητές σε επίπεδο δωματίου, συμπεριλαμβανομένων ανώνυμων θεατών που δεν απαριθμούμε).

## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό διεύθυνσης URL της σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | No | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | No | Διαχωριστής ισοβαθμίας του δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες κατά όνομα να μην παραλείπουν εγγραφές. |

## Απόκριση

Επιστρέφει: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Εισαγωγή κλάσεων:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | Αναγνωριστικό διεύθυνσης URL της σελίδας (καθαρίζεται από τον διακομιστή).
    String afterName = "afterName_example"; // String | Δείκτης: περάστε το nextAfterName από την προηγούμενη απόκριση.
    String afterUserId = "afterUserId_example"; // String | Διαχωριστής ισοβαθμίας του δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι ισοβαθμίες κατά όνομα να μην παραλείπουν εγγραφές.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]