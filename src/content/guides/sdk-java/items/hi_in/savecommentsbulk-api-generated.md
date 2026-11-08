## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| isLive | boolean | query | नहीं |  |
| doSpamCheck | boolean | query | नहीं |  |
| sendEmails | boolean | query | नहीं |  |
| populateNotifications | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SaveCommentsBulkResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'saveCommentsBulk उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लासेस इंपोर्ट करें:
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
    List<CreateCommentParams> createCommentParams = Arrays.asList(); // List<CreateCommentParams> | 
    Boolean isLive = true; // Boolean | 
    Boolean doSpamCheck = true; // Boolean | 
    Boolean sendEmails = true; // Boolean | 
    Boolean populateNotifications = true; // Boolean | 
    try {
      List<SaveCommentsBulkResponse> result = apiInstance.saveCommentsBulk(tenantId, createCommentParams)
            .isLive(isLive)
            .doSpamCheck(doSpamCheck)
            .sendEmails(sendEmails)
            .populateNotifications(populateNotifications)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("DefaultApi#saveCommentsBulk कॉल करते समय अपवाद");
      System.err.println("स्टेटस कोड: " + e.getCode());
      System.err.println("कारण: " + e.getResponseBody());
      System.err.println("रिस्पॉन्स हेडर: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]