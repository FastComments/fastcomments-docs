किसी कमेंट से पोल जोड़ें, या उसके पहले से मौजूद पोल की पूरी स्थिति सेट करें।

विकल्प id द्वारा मैच किए जाते हैं: किसी मौजूदा विकल्प की id के साथ भेजा गया विकल्प अपने वोट बरकरार रखता है (और नया लेबल तथा स्थान ले लेता है), बिना id भेजा गया विकल्प जोड़ दिया जाता है, और सूची से बाहर छोड़े गए मौजूदा विकल्प उन पर डाले गए वोटों सहित हटा दिए जाते हैं।

वोट वाले पोल पर कोई भी मौजूदा विकल्प id न रखने पर उन सभी को हटा दिया जाता है, इसलिए इसके लिए replaceVotes=true की आवश्यकता होती है।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentId | string | path | हाँ |  |
| replaceVotes | boolean | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`SavePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SavePollResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'putPoll उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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