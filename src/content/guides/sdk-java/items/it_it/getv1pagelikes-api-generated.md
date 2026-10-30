## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Quando impostato, restituisce gli utenti che hanno messo "Mi piace" alla pagina (avatar e nomi utente). È accettato solo "public", che omette i profili privati. |

## Risposta

Restituisce: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV1PageLikes.java)

## Esempio

[inline-code-attrs-start title = 'Esempio getV1PageLikes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa le classi:
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
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | Quando impostato, restituisce gli utenti che hanno messo "Mi piace" alla pagina (avatar e nomi utente). È accettato solo \"public\", che omette i profili privati.
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