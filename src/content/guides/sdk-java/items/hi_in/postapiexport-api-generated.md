## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| text-search | string | query | नहीं |  |
| byIPFromComment | string | query | नहीं |  |
| filters | string | query | नहीं |  |
| searchFilters | string | query | नहीं |  |
| sorts | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationExportResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'postApiExport उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लास इम्पोर्ट करें:
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
    String textSearch = "textSearch_example"; // String | 
    String byIPFromComment = "byIPFromComment_example"; // String | 
    String filters = "filters_example"; // String | 
    String searchFilters = "searchFilters_example"; // String | 
    String sorts = "sorts_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationExportResponse result = apiInstance.postApiExport(tenantId)
            .textSearch(textSearch)
            .byIPFromComment(byIPFromComment)
            .filters(filters)
            .searchFilters(searchFilters)
            .sorts(sorts)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postApiExport");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]