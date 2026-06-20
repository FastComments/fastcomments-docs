Trenutno prisotni gledalci strani: osebe, katerih websocket seja je trenutno naročena na to stran.
Vrne anonCount + totalCount (naročniki v celotni sobi, vključno z anonimnimi gledalci, ki jih ne navajamo).

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL strani (očiščen na strežniku). |
| afterName | string | query | No | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kazalec za razreševanje izenačitev: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, kadar je afterName nastavljeno, da se vnosi z enakimi imeni ne izgubijo. |

## Odgovor

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    String urlId = "urlId_example"; // String | Identifikator URL strani (očiščen na strežniku).
    String afterName = "afterName_example"; // String | Kazalec: posredujte nextAfterName iz prejšnjega odgovora.
    String afterUserId = "afterUserId_example"; // String | Kazalec za razreševanje izenačitev: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, kadar je afterName nastavljeno, da se vnosi z enakimi imeni ne izgubijo.
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