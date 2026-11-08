## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| includeByUserIdAndEmail | boolean | query | नहीं |  |
| includeByIP | boolean | query | नहीं |  |
| includeByEmailDomain | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BulkPreBanSummary.java)

## उदाहरण

[inline-code-attrs-start title = 'postBulkPreBanSummary उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लासेस इम्पोर्ट करें:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    BulkPreBanParams bulkPreBanParams = new BulkPreBanParams(); // BulkPreBanParams | 
    Boolean includeByUserIdAndEmail = true; // Boolean | 
    Boolean includeByIP = true; // Boolean | 
    Boolean includeByEmailDomain = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      BulkPreBanSummary result = apiInstance.postBulkPreBanSummary(tenantId, bulkPreBanParams)
            .includeByUserIdAndEmail(includeByUserIdAndEmail)
            .includeByIP(includeByIP)
            .includeByEmailDomain(includeByEmailDomain)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postBulkPreBanSummary");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]