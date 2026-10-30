Iscrive un URL a un evento di commento (sottoscrizione REST hook). Iscrivere nuovamente lo stesso URL allo stesso evento e dominio restituisce l'iscrizione esistente. Le consegne sono firmate con HMAC, vedere la guida sui webhook; l'intestazione legacy `token` non viene mai inviata alle iscrizioni API.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateWebhookResponse.java)

## Example

[inline-code-attrs-start title = 'Esempio createWebhook'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad es. "Token" (predefinito a null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // Stringa |
    CreateWebhookParams createWebhookParams = new CreateWebhookParams(); // CreateWebhookParams |
    try {
      CreateWebhookResponse result = apiInstance.createWebhook(tenantId, createWebhookParams)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Eccezione durante la chiamata a DefaultApi#createWebhook");
      System.err.println("Codice di stato: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Intestazioni della risposta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]