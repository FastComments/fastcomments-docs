Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή συνδεδεμένοι. Ταξινομημένο κατά displayName.
Χρησιμοποιήστε αυτό αφού εξαντλήσετε το /users/online για να εμφανίσετε μια ενότητα "Μέλη".
Σελιδοποίηση cursor βάσει του commenterName: ο διακομιστής διασχίζει το μερικό {tenantId, urlId, commenterName}
ευρετήριο από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | διαδρομή | Ναι |  |
| urlId | string | query | Ναι | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή). |
| afterName | string | query | Όχι | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απόκριση. |
| afterUserId | string | query | Όχι | Διαιτητής ισοβαθμίας του δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην παραλείπουν εγγραφές. |

## Απόκριση

Επιστρέφει: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Αναγνωριστικό URL σελίδας (καθαρίζεται από τον διακομιστή).
    String afterName = "afterName_example"; // String | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απόκριση.
    String afterUserId = "afterUserId_example"; // String | Διαιτητής ισοβαθμίας του δείκτη: περάστε το nextAfterUserId από την προηγούμενη απόκριση. Απαιτείται όταν το afterName έχει οριστεί ώστε οι ισοβαθμίες ονομάτων να μην παραλείπουν εγγραφές.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]