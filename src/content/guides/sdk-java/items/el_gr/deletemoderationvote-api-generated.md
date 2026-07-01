## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| voteId | string | path | Ναι |  |
| broadcastId | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Response

Επιστρέφει: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VoteDeleteResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deleteModerationVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
/* Εισαγωγή κλάσεων: */
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
    String commentId = "commentId_example"; // String | 
    String voteId = "voteId_example"; // String | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      VoteDeleteResponse result = apiInstance.deleteModerationVote(tenantId, commentId, voteId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Εξαίρεση κατά την κλήση του ModerationApi#deleteModerationVote");
      System.err.println("Κωδικός κατάστασης: " + e.getCode());
      System.err.println("Αιτία: " + e.getResponseBody());
      System.err.println("Κεφαλίδες απόκρισης: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]