## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| questionId | string | query | नहीं |  |
| questionIds | array | query | नहीं |  |
| urlId | string | query | नहीं |  |
| timeBucket | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| forceRecalculate | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AggregateQuestionResultsResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'aggregateQuestionResults उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लासेस इम्पोर्ट करें:
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
    
    // API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // API की के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित लाइन को अनकमेंट करें, जैसे "Token" (डिफ़ॉल्ट null है)
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
      System.err.println("Exception when calling DefaultApi#aggregateQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]