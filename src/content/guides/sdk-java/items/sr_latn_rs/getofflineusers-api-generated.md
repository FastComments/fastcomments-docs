---
Prethodni komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon iscrpljivanja /users/online da biste prikazali sekciju "Members".
Kursor paginacija po commenterName: server pretražuje delimični indeks {tenantId, urlId, commenterName}
od afterName unapred koristeći $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (očišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: prosledi nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrešavanje izjednačenja: prosledi nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen da se unosi sa istim imenom ne bi bili ispustili. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Identifikator URL stranice (očišćen na serverskoj strani).
    String afterName = "afterName_example"; // String | Kursor: prosledi nextAfterName iz prethodnog odgovora.
    String afterUserId = "afterUserId_example"; // String | Kursor za razrešavanje izjednačenja: prosledi nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen da se unosi sa istim imenom ne bi bili ispustili.
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