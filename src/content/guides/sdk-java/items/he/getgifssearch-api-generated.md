## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| search | string | query | כן |  |
| locale | string | query | לא |  |
| rating | string | query | לא |  |
| page | number | query | לא |  |

## תגובה

מחזיר: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetGifsSearchResponse.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getGifsSearch'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String search = "search_example"; // String | 
    String locale = "locale_example"; // String | 
    String rating = "rating_example"; // String | 
    Double page = 3.4D; // Double | 
    try {
      GetGifsSearchResponse result = apiInstance.getGifsSearch(tenantId, search)
            .locale(locale)
            .rating(rating)
            .page(page)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getGifsSearch");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---