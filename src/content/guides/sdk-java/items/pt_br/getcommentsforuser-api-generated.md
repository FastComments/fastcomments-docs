## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| direction | string | query | Não |  |
| repliesToUserId | string | query | Não |  |
| page | number | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| isCrawler | boolean | query | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentsForUserResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsForUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
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