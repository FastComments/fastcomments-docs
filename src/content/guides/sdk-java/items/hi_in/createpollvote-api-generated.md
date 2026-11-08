पोल पर वोट दर्ज करें, या किसी मौजूदा वोट को दूसरे विकल्प पर स्थानांतरित करें। एक मतदाता का प्रति पोल अधिकतम एक वोट होता है, इसलिए उसी मतदाता के लिए इसे दोबारा कॉल करने पर नया वोट जोड़ने के बजाय उनका वोट स्थानांतरित हो जाता है।

यह साइट की पोल सेटिंग्स का पालन करता है: यदि वोटिंग केवल लॉग-इन उपयोगकर्ताओं के लिए सेट है, तो केवल anonUserId वाला वोट अस्वीकार कर दिया जाता है, और गुमनाम वोट प्रति IP प्रति पोल रेट-लिमिटेड होते हैं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`CreatePollVoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreatePollVoteResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'createPollVote उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    CreatePollVoteBody createPollVoteBody = new CreatePollVoteBody(); // CreatePollVoteBody | 
    try {
      CreatePollVoteResponse result = apiInstance.createPollVote(tenantId, createPollVoteBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createPollVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]