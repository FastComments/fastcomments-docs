---
Użytkownicy, którzy komentowali stronę i którzy NIE są aktualnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyrenderować sekcję "Members".
Stronicowanie kursorowe po commenterName: serwer przeszukuje częściowy indeks {tenantId, urlId, commenterName}
index od afterName w przód przy użyciu $gt, bez kosztu $skip.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi. |
| afterUserId | string | query | No | Kursor jako rozstrzygacz remisów: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów. |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importuj klasy:
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
    String urlId = "urlId_example"; // String | Identyfikator adresu URL strony (oczyszczany po stronie serwera).
    String afterName = "afterName_example"; // String | Kursor: przekaż nextAfterName z poprzedniej odpowiedzi.
    String afterUserId = "afterUserId_example"; // String | Kursor jako rozstrzygacz remisów: przekaż nextAfterUserId z poprzedniej odpowiedzi. Wymagane, gdy afterName jest ustawione, aby remisy nazw nie powodowały pominięcia wpisów.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---