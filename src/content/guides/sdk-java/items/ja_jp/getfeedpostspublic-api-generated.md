req
tenantId
afterId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| afterId | string | query | No |  |
| limit | integer | query | No |  |
| tags | array | query | No |  |
| sso | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeUserInfo | boolean | query | No |  |

## レスポンス

戻り値: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetFeedPostsPublic200Response.java)

## 例

[inline-code-attrs-start title = 'getFeedPostsPublic の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
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