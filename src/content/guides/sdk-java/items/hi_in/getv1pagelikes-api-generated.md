## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| sso | string | query | नहीं |  |
| includeUserInfo | string | query | नहीं | सेट होने पर, पेज को लाइक करने वाले उपयोगकर्ताओं को लौटाता है (अवतार और उपयोगकर्ता नाम)। केवल "public" स्वीकार किया जाता है, जो निजी प्रोफ़ाइलों को छोड़ देता है। |

## प्रतिक्रिया

लौटाता है: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV1PageLikes.java)

## उदाहरण

[inline-code-attrs-start title = 'getV1PageLikes उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | सेट होने पर, पेज को लाइक करने वाले उपयोगकर्ताओं को लौटाता है (अवतार और उपयोगकर्ता नाम)। केवल \"public\" स्वीकार किया जाता है, जो निजी प्रोफ़ाइलों को छोड़ देता है।
    try {
      GetV1PageLikes result = apiInstance.getV1PageLikes(tenantId, urlId)
            .sso(sso)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getV1PageLikes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]