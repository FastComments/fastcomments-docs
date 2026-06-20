## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| text-search | string | query | Όχι |  |
| byIPFromComment | string | query | Όχι |  |
| filters | string | query | Όχι |  |
| searchFilters | string | query | Όχι |  |
| afterId | string | query | Όχι |  |
| demo | boolean | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPIGetCommentIdsResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getApiIds'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Εισαγωγή κλάσεων:
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
    String filters = "filters_example"; // String | 
    String searchFilters = "searchFilters_example"; // String | 
    String afterId = "afterId_example"; // String | 
    Boolean demo = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPIGetCommentIdsResponse result = apiInstance.getApiIds()
            .textSearch(textSearch)
            .byIPFromComment(byIPFromComment)
            .filters(filters)
            .searchFilters(searchFilters)
            .afterId(afterId)
            .demo(demo)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getApiIds");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]