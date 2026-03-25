## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| usernameStartsWith | string | query | Ne |  |
| mentionGroupIds | array | query | Ne |  |
| sso | string | query | Ne |  |
| searchSection | string | query | Ne |  |

## Odgovor

Vrne: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SearchUsers200Response.java)

## Primer

[inline-code-attrs-start title = 'searchUsers Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String usernameStartsWith = "usernameStartsWith_example"; // String | 
    List<String> mentionGroupIds = Arrays.asList(); // List<String> | 
    String sso = "sso_example"; // String | 
    String searchSection = "fast"; // String | 
    try {
      SearchUsers200Response result = apiInstance.searchUsers(tenantId, urlId)
            .usernameStartsWith(usernameStartsWith)
            .mentionGroupIds(mentionGroupIds)
            .sso(sso)
            .searchSection(searchSection)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#searchUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---