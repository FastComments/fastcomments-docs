## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postIds | array | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetFeedPostsStats200Response.java)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsStats Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    List<String> postIds = Arrays.asList(); // List<String> | 
    String sso = "sso_example"; // String | 
    try {
      GetFeedPostsStats200Response result = apiInstance.getFeedPostsStats(tenantId, postIds)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getFeedPostsStats");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---