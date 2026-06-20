## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filter | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Geeft terug: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPICountCommentsResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getCount Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    String textSearch = "textSearch_example"; // String | 
    String byIPFromComment = "byIPFromComment_example"; // String | 
    String filter = "filter_example"; // String | 
    String searchFilters = "searchFilters_example"; // String | 
    Boolean demo = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPICountCommentsResponse result = apiInstance.getCount()
            .textSearch(textSearch)
            .byIPFromComment(byIPFromComment)
            .filter(filter)
            .searchFilters(searchFilters)
            .demo(demo)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---