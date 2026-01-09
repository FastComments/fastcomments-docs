בקשה
tenantId
afterId

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| afterId | string | query | לא |  |
| limit | integer | query | לא |  |
| tags | array | query | לא |  |
| sso | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeUserInfo | boolean | query | לא |  |

## תגובה

מחזיר: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetFeedPostsPublic200Response.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getFeedPostsPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// ייבוא מחלקות:
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
    String afterId = "afterId_example"; // String | 
    Integer limit = 56; // Integer | 
    List<String> tags = Arrays.asList(); // List<String> | 
    String sso = "sso_example"; // String | 
    Boolean isCrawler = true; // Boolean | 
    Boolean includeUserInfo = true; // Boolean | 
    try {
      GetFeedPostsPublic200Response result = apiInstance.getFeedPostsPublic(tenantId)
            .afterId(afterId)
            .limit(limit)
            .tags(tags)
            .sso(sso)
            .isCrawler(isCrawler)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getFeedPostsPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]