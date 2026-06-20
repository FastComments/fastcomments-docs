## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| questionId | string | query | Όχι |  |
| questionIds | array | query | Όχι |  |
| urlId | string | query | Όχι |  |
| timeBucket | string | query | Όχι |  |
| startDate | string | query | Όχι |  |
| forceRecalculate | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AggregateQuestionResultsResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregateQuestionResults'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (προεπιλογή: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String questionId = "questionId_example"; // String | 
    List<String> questionIds = Arrays.asList(); // List<String> | 
    String urlId = "urlId_example"; // String | 
    AggregateTimeBucket timeBucket = AggregateTimeBucket.fromValue("day"); // AggregateTimeBucket | 
    OffsetDateTime startDate = OffsetDateTime.now(); // OffsetDateTime | 
    Boolean forceRecalculate = true; // Boolean | 
    try {
      AggregateQuestionResultsResponse result = apiInstance.aggregateQuestionResults(tenantId)
            .questionId(questionId)
            .questionIds(questionIds)
            .urlId(urlId)
            .timeBucket(timeBucket)
            .startDate(startDate)
            .forceRecalculate(forceRecalculate)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Εξαίρεση κατά την κλήση DefaultApi#aggregateQuestionResults");
      System.err.println("Κωδικός κατάστασης: " + e.getCode());
      System.err.println("Αιτία: " + e.getResponseBody());
      System.err.println("Επικεφαλίδες απόκρισης: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]