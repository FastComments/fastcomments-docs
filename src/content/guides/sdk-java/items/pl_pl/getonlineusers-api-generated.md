Aktualnie online widzowie strony: osoby, których sesja websocket jest obecnie subskrybowana na tę stronę.
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, włączając anonimowych widzów, których nie wymieniamy).

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identyfikator URL strony (oczyszczany po stronie serwera). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Kursor — wartość rozstrzygająca: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby przy remisach nazw nie pomijać wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importowanie klas:
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
    String urlId = "urlId_example"; // String | Identyfikator URL strony (oczyszczany po stronie serwera).
    String afterName = "afterName_example"; // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
    String afterUserId = "afterUserId_example"; // String | Wartość rozstrzygająca kursora (tiebreaker): przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby przy remisach nazw nie pomijać wpisów.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]