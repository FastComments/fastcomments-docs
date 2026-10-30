Conteggi personalizzati delle reazioni per una pagina, opzionalmente con gli utenti che hanno reagito.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | Sì |  |
| includeUserInfo | string | query | No | "public" omette gli utenti con profili privati, "include-private" li include. |

## Risposta

Restituisce: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageReactsAPIResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio getPageReacts'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa le classi:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // Configura l'autorizzazione della chiave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad esempio "Token" (predefinito a null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // Stringa |
    String urlId = "urlId_example"; // Stringa |
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | \"public\" omette gli utenti con profili privati, \"include-private\" li include.
    try {
      GetPageReactsAPIResponse result = apiInstance.getPageReacts(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPageReacts");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---