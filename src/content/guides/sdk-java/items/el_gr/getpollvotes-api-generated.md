Οι μεμονωμένες ψήφοι πίσω από τα σύνολα μιας ψηφοφορίας, ξεκινώντας από τις παλαιότερες.

Μια ψηφοφορία ανήκει σε ένα σχόλιο, οπότε οι ψήφοι διαβάζονται πάντα μία ψηφοφορία τη φορά - το commentId είναι υποχρεωτικό. Αυτό
διατηρεί κάθε ερώτημα στα ευρετήρια που έχει ήδη η συλλογή.

Τηρεί την ιδιωτικότητα της ψηφοφορίας: οι ψήφοι μιας ανώνυμης ψηφοφορίας δεν μπορούν να διαβαστούν (poll-anonymous), ούτε εδώ ούτε μέσω id.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | query | Ναι |  |
| voterId | string | query | Όχι |  |
| optionId | string | query | Όχι |  |
| skip | integer | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVotesResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPollVotes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String voterId = "voterId_example"; // String | 
    String optionId = "optionId_example"; // String | 
    Integer skip = 56; // Integer | 
    try {
      GetPollVotesResponse result = apiInstance.getPollVotes(tenantId, commentId)
            .voterId(voterId)
            .optionId(optionId)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPollVotes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]