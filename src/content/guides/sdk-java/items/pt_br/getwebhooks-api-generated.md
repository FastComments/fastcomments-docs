Lists the webhooks configurados para o tenant, tanto linhas gerenciadas pelo painel quanto assinaturas de API.

## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| domain | string | query | No |  |
| source | string | query | No |  |
| skip | number | query | No |  |

## Response

Retorna: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetWebhooksResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getWebhooks'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
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
    
    // Configurar autorização da chave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente a linha a seguir para definir um prefixo para a chave API, por exemplo "Token" (padrão null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    WebhookEventName event = WebhookEventName.fromValue("comment-created"); // WebhookEventName | 
    String domain = "domain_example"; // String | 
    WebhookSource source = WebhookSource.fromValue("dashboard"); // WebhookSource | 
    Double skip = 3.4D; // Double | 
    try {
      GetWebhooksResponse result = apiInstance.getWebhooks(tenantId)
            .event(event)
            .domain(domain)
            .source(source)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getWebhooks");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---