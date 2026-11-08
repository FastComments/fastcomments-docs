एक पोल की गिनती के पीछे के व्यक्तिगत वोट, सबसे पुराने पहले।

एक पोल एक कमेंट से संबंधित होता है, इसलिए वोट हमेशा एक बार में एक पोल के लिए ही पढ़े जाते हैं - commentId आवश्यक है। इससे हर क्वेरी उन इंडेक्स पर ही चलती है जो कलेक्शन में पहले से मौजूद हैं।

यह पोल की प्राइवेसी का पालन करता है: एक अनाम (anonymous) पोल के वोट नहीं पढ़े जा सकते (poll-anonymous), न यहाँ और न ही id द्वारा।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | query | हाँ |  |
| voterId | string | query | नहीं |  |
| optionId | string | query | नहीं |  |
| skip | integer | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetPollVotesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVotesResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getPollVotes उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लास इम्पोर्ट करें:
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
    
    // API key authorization कॉन्फ़िगर करें: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // API key के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित लाइन को अनकमेंट करें, जैसे "Token" (डिफ़ॉल्ट null है)
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