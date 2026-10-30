## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nein |  |
| includeUserInfo | string | query | Nein | Wenn gesetzt, werden die Benutzer zurückgegeben, die die Seite geliked haben (Avatare und Benutzernamen). Nur „public“ wird akzeptiert, wodurch private Profile weggelassen werden. |

## Antwort

Rückgabe: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV1PageLikes.java)

## Beispiel

[inline-code-attrs-start title = 'getV1PageLikes Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importieren von Klassen:
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
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | Wenn gesetzt, werden die Benutzer zurückgegeben, die die Seite geliked haben (Avatare und Benutzernamen). Nur \"public\" wird akzeptiert, wodurch private Profile weggelassen werden.
    try {
      GetV1PageLikes result = apiInstance.getV1PageLikes(tenantId, urlId)
            .sso(sso)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getV1PageLikes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]