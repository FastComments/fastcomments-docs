## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Wenn gesetzt, werden die Benutzer für jede Reaktions‑ID zurückgegeben (Avatare und Benutzernamen). Nur "public" wird akzeptiert, wodurch private Profile weggelassen werden. |

## Antwort

Rückgabe: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV2PageReacts.java)

## Beispiel

[inline-code-attrs-start title = 'getV2PageReacts Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | Wenn gesetzt, werden die Benutzer für jede Reaktions‑ID zurückgegeben (Avatare und Benutzernamen). Nur \"public\" wird akzeptiert, wodurch private Profile weggelassen werden.
    try {
      GetV2PageReacts result = apiInstance.getV2PageReacts(tenantId, urlId)
            .sso(sso)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Ausnahme beim Aufruf von PublicApi#getV2PageReacts");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Grund: " + e.getResponseBody());
      System.err.println("Antwort-Header: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---