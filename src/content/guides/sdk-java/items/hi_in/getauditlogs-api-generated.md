## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ | वह टेनेंट जिसके लिए ऑडिट लॉग पढ़ने हैं। |
| limit | integer | query | नहीं | कितने लॉग लौटाने हैं। अधिकतम 10000, डिफ़ॉल्ट 1000। |
| skip | integer | query | नहीं | पेजिंग के लिए कितने लॉग छोड़ने हैं। |
| order | string | query | नहीं | इवेंट समय पर सॉर्ट दिशा। डिफ़ॉल्ट DESC (नवीनतम पहले)। |
| after | integer | query | नहीं |  |
| before | integer | query | नहीं |  |
| username | string | query | नहीं | केवल इस यूज़रनेम द्वारा किए गए इवेंट लौटाएँ। |
| ip | string | query | नहीं | केवल इस IP पते से इवेंट लौटाएँ। |
| crudType | string | query | नहीं | केवल इस प्रकार के इवेंट लौटाएँ: c, r, u, d, या login। |
| resourceName | string | query | नहीं | केवल इस रिसोर्स के लिए इवेंट लौटाएँ, जैसे User या Moderator। |
| targetId | string | query | नहीं | केवल वे इवेंट लौटाएँ जिनकी प्रभावित वस्तु की यह id है। |
| target | string | query | नहीं | प्रभावित वस्तु के लेबल पर केस-असंवेदनशील सबस्ट्रिंग मैच, जैसे यूज़रनेम या ईमेल। |
| includeManagedTenants | boolean | query | नहीं | उन टेनेंट्स के इवेंट भी लौटाएँ जिन्हें यह टेनेंट प्रबंधित करता है। |

## प्रतिक्रिया

लौटाता है: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getAuditLogs उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // String | वह टेनेंट जिसके लिए ऑडिट लॉग पढ़ने हैं।
    Integer limit = 56; // Integer | कितने लॉग लौटाने हैं। अधिकतम 10000, डिफ़ॉल्ट 1000।
    Integer skip = 56; // Integer | पेजिंग के लिए कितने लॉग छोड़ने हैं।
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | इवेंट समय पर सॉर्ट दिशा। डिफ़ॉल्ट DESC (नवीनतम पहले)।
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | केवल इस यूज़रनेम द्वारा किए गए इवेंट लौटाएँ।
    String ip = "ip_example"; // String | केवल इस IP पते से इवेंट लौटाएँ।
    String crudType = "crudType_example"; // String | केवल इस प्रकार के इवेंट लौटाएँ: c, r, u, d, या login।
    String resourceName = "resourceName_example"; // String | केवल इस रिसोर्स के लिए इवेंट लौटाएँ, जैसे User या Moderator।
    String targetId = "targetId_example"; // String | केवल वे इवेंट लौटाएँ जिनकी प्रभावित वस्तु की यह id है।
    String target = "target_example"; // String | प्रभावित वस्तु के लेबल पर केस-असंवेदनशील सबस्ट्रिंग मैच, जैसे यूज़रनेम या ईमेल।
    Boolean includeManagedTenants = true; // Boolean | उन टेनेंट्स के इवेंट भी लौटाएँ जिन्हें यह टेनेंट प्रबंधित करता है।
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]