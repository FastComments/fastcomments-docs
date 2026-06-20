Προηγούμενοι σχολιαστές στη σελίδα που ΔΕΝ είναι αυτή τη στιγμή online. Ταξινομημένο κατά displayName.
Χρησιμοποιήστε το αφού εξαντλήσετε το /users/online για να αποδώσετε μια ενότητα "Μέλη".
Σελιδοποίηση cursor με βάση το commenterName: ο server περπατάει τον μερικό {tenantId, urlId, commenterName}
index από afterName προς τα εμπρός μέσω $gt, χωρίς κόστος $skip.

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Αναγνωριστικό URL σελίδας (καθαρίζεται στην πλευρά του διακομιστή). |
| afterName | string | query | No | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση. |
| afterUserId | string | query | No | Επίλυση ισοβαθμίας δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι εγγραφές με ίδια ονόματα να μην παραλείπονται. |

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
    String urlId = "urlId_example"; // String | Αναγνωριστικό URL σελίδας (καθαρίζεται στην πλευρά του διακομιστή).
    String afterName = "afterName_example"; // String | Δείκτης (cursor): περάστε το nextAfterName από την προηγούμενη απάντηση.
    String afterUserId = "afterUserId_example"; // String | Επίλυση ισοβαθμίας δείκτη: περάστε το nextAfterUserId από την προηγούμενη απάντηση. Απαιτείται όταν έχει οριστεί το afterName ώστε οι εγγραφές με ίδια ονόματα να μην παραλείπονται.
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