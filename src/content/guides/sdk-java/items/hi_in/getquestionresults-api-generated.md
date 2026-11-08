## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं |  |
| userId | string | query | नहीं |  |
| startDate | string | query | नहीं |  |
| questionId | string | query | नहीं |  |
| questionIds | string | query | नहीं |  |
| skip | number | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetQuestionResultsResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getQuestionResults उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String userId = "userId_example"; // String | 
    String startDate = "startDate_example"; // String | 
    String questionId = "questionId_example"; // String | 
    String questionIds = "questionIds_example"; // String | 
    Double skip = 3.4D; // Double | 
    try {
      GetQuestionResultsResponse result = apiInstance.getQuestionResults(tenantId)
            .urlId(urlId)
            .userId(userId)
            .startDate(startDate)
            .questionId(questionId)
            .questionIds(questionIds)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("DefaultApi#getQuestionResults कॉल करते समय अपवाद");
      System.err.println("स्थिति कोड: " + e.getCode());
      System.err.println("कारण: " + e.getResponseBody());
      System.err.println("प्रतिक्रिया हेडर: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]