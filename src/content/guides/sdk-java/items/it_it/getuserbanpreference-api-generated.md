## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIModerateGetUserBanPreferencesResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserBanPreference'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa classi:
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
    String sso = "sso_example"; // String | 
    try {
      APIModerateGetUserBanPreferencesResponse result = apiInstance.getUserBanPreference(tenantId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Eccezione durante la chiamata a ModerationApi#getUserBanPreference");
      System.err.println("Codice di stato: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Intestazioni della risposta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]