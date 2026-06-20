## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| userId | string | query | No |  |
| direction | string | query | No |  |
| repliesToUserId | string | query | No |  |
| page | number | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| isCrawler | boolean | query | No |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentsForUserResponse.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getCommentsForUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    String userId = "userId_example"; // String | 
    SortDirections direction = SortDirections.fromValue("OF"); // SortDirections | 
    String repliesToUserId = "repliesToUserId_example"; // String | 
    Double page = 3.4D; // Double | 
    Boolean includei10n = true; // Boolean | 
    String locale = "locale_example"; // String | 
    Boolean isCrawler = true; // Boolean | 
    try {
      GetCommentsForUserResponse result = apiInstance.getCommentsForUser()
            .userId(userId)
            .direction(direction)
            .repliesToUserId(repliesToUserId)
            .page(page)
            .includei10n(includei10n)
            .locale(locale)
            .isCrawler(isCrawler)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getCommentsForUser");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---