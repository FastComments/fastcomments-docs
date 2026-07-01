## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| tag | string | path | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio deleteHashTag'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad es. "Token" (predefinito null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String tag = "tag_example"; // String | 
    DeleteHashTagRequestBody deleteHashTagRequestBody = new DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 
    try {
      APIEmptyResponse result = apiInstance.deleteHashTag(tenantId, tag)
            .deleteHashTagRequestBody(deleteHashTagRequestBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Eccezione durante la chiamata a DefaultApi#deleteHashTag");
      System.err.println("Codice di stato: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Intestazioni della risposta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---