किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सत्र इस समय पेज की सदस्यता लिए हुए है।
anonCount + totalCount लौटाता है (पूरे रूम के सदस्य, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम सूचीबद्ध नहीं करते)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ छूट न जाएँ। |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getOnlineUsers उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लासेस इम्पोर्ट करें:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
    String afterName = "afterName_example"; // String | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
    String afterUserId = "afterUserId_example"; // String | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। आवश्यक है जब afterName सेट हो ताकि समान नाम वाली प्रविष्टियाँ छूट न जाएँ।
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]