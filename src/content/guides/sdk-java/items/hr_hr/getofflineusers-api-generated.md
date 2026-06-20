Prethodni komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online kako biste prikazali odjeljak "Članovi".
Cursor paginacija po commenterName: server prolazi djelomični indeks {tenantId, urlId, commenterName}
od afterName naprijed putem $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (čišćen na strani servera). |
| afterName | string | query | No | Cursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Tie-breaker cursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se unosi s istim imenom ne bi izostali. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Primjer

[inline-code-attrs-start title = 'Primjer getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvoz klasa:
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
    String urlId = "urlId_example"; // String | Identifikator URL stranice (čišćen na strani servera).
    String afterName = "afterName_example"; // String | Cursor: proslijedite nextAfterName iz prethodnog odgovora.
    String afterUserId = "afterUserId_example"; // String | Tie-breaker cursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se unosi s istim imenom ne bi izostali.
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