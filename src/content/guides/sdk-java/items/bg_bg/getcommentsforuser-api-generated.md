## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | Не |  |
| direction | string | query | Не |  |
| repliesToUserId | string | query | Не |  |
| page | number | query | Не |  |
| includei10n | boolean | query | Не |  |
| locale | string | query | Не |  |
| isCrawler | boolean | query | Не |  |

## Отговор

Връща: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentsForUserResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsForUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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