## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | Yes |  |
| direction | string | query | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## Απόκριση

Επιστρέφει: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VoteComment200Response.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε ένα πρόθεμα για το κλειδί API, π.χ. "Token" (από προεπιλογή null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String direction = "up"; // String | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    try {
      VoteComment200Response result = apiInstance.createVote(tenantId, commentId, direction)
            .userId(userId)
            .anonUserId(anonUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]