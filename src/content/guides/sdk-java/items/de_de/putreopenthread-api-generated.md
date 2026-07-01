## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Beispiel

[inline-code-attrs-start title = 'putReopenThread Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.putReopenThread(tenantId, urlId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Ausnahme beim Aufruf von ModerationApi#putReopenThread
      System.err.println("Exception when calling ModerationApi#putReopenThread");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]