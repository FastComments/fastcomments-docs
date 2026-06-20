Trenutno prisutni gledaoci stranice: osobe čija je websocket sesija trenutno pretplaćena na tu stranicu.
Vraća anonCount + totalCount (pretplatnici sobe, uključujući anonimne gledaoce koje ne izlistavamo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (očišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrešavanje nerešenih: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se izbeglo izostavljanje zapisa sa istim imenom. |

## Response

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Example

[inline-code-attrs-start title = 'getOnlineUsers Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Identifikator URL-a stranice (očišćen na serverskoj strani).
    String afterName = "afterName_example"; // String | Kursor: prosledite nextAfterName iz prethodnog odgovora.
    String afterUserId = "afterUserId_example"; // String | Kursor za razrešavanje nerešenih: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se izbeglo izostavljanje zapisa sa istim imenom.
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