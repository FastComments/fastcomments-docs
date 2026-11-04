Επισυνάψτε μια δημοσκόπηση σε ένα σχόλιο, ή ορίστε την πλήρη κατάσταση της δημοσκόπησης που ήδη έχει.

Οι επιλογές αντιστοιχίζονται με βάση το id: μια επιλογή που αποστέλλεται με το id μιας υπάρχουσας επιλογής διατηρεί τις ψήφους της (και λαμβάνει τη νέα ετικέτα και θέση), μια επιλογή που αποστέλλεται χωρίς id προστίθεται, και οι υπάρχουσες επιλογές που παραλείπονται από τη λίστα αφαιρούνται μαζί με τις ψήφους που έχουν καταψηφιστεί σε αυτές.

Η διατήρηση καμία υπάρχοντος id επιλογής σε μια δημοσκόπηση που έχει ψήφους διαγράφει όλες αυτές, οπότε αυτό απαιτεί replaceVotes=true.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | path | Ναι |  |
| replaceVotes | boolean | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα putPoll'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    CommentPollPutInput commentPollPutInput = new CommentPollPutInput(); // CommentPollPutInput | 
    Boolean replaceVotes = true; // Boolean | 
    try {
      SavePollResponse result = apiInstance.putPoll(tenantId, commentId, commentPollPutInput)
            .replaceVotes(replaceVotes)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#putPoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]