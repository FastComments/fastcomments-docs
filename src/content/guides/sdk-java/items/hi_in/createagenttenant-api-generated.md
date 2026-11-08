किसी AI एजेंट के लिए बिना मानव साइनअप के एक नया ट्रायल अकाउंट बनाता है। इसे कॉल करने के लिए किसी API key की आवश्यकता नहीं है।

रिस्पॉन्स में tenant id, एक API key शामिल है जो तुरंत REST API और MCP server के खिलाफ काम करती है, और एक claim URL। Claim URL उस व्यक्ति को दें जिसके लिए आप काम कर रहे हैं: FastComments में लॉग इन होने के दौरान इसे खोलने पर अकाउंट उससे जुड़ जाता है। Unclaimed अकाउंट, और उनकी keys, बनाए जाने के 72 घंटे बाद डिलीट कर दिए जाते हैं। Claim होने तक, अकाउंट पर मानक ट्रायल सीमाएँ लागू रहती हैं।

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'createAgentTenant उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]