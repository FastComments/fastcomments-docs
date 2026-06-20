## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| page | number | query | Non |  |
| count | number | query | Non |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sorts | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPIGetCommentsResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getApiComments'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    Double page = 3.4D; // Double | 
    Double count = 3.4D; // Double | 
    String textSearch = "textSearch_example"; // String | 
    String byIPFromComment = "byIPFromComment_example"; // String | 
    String filters = "filters_example"; // String | 
    String searchFilters = "searchFilters_example"; // String | 
    String sorts = "sorts_example"; // String | 
    Boolean demo = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPIGetCommentsResponse result = apiInstance.getApiComments()
            .page(page)
            .count(count)
            .textSearch(textSearch)
            .byIPFromComment(byIPFromComment)
            .filters(filters)
            .searchFilters(searchFilters)
            .sorts(sorts)
            .demo(demo)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getApiComments");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---