req
tenantId
afterId

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| afterId | string | query | Nee |  |
| limit | integer | query | Nee |  |
| tags | array | query | Nee |  |
| sso | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeUserInfo | boolean | query | Nee |  |

## Antwoord

Retourneert: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetFeedPostsPublic200Response.java)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsPublic Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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