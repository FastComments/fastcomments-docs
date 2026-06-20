---
Trenutno online gledatelji stranice: osobe čija je websocket sesija trenutno pretplaćena na tu stranicu.
Vraća anonCount + totalCount (pretplatnici u sobi, uključujući anonimne gledatelje koje ne navodimo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na strani servera). |
| afterName | string | query | Ne | Kursor: pošaljite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tie-breaker kursora: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi s istim imenom ne bi izostavili. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Primjer

[inline-code-attrs-start title = 'getOnlineUsers Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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
    String urlId = "urlId_example"; // String | Identifikator URL-a stranice (očišćen na strani servera).
    String afterName = "afterName_example"; // String | Kursor: pošaljite nextAfterName iz prethodnog odgovora.
    String afterUserId = "afterUserId_example"; // String | Tie-breaker kursora: pošaljite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi s istim imenom ne bi izostavili.
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

---