## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|--------------|-----------|
| tenantId | string | query | Yes | O tenant para ler os logs de auditoria. |
| limit | integer | query | No | Quantos logs retornar. Máximo 10000, padrão 1000. |
| skip | integer | query | No | Quantos logs pular, para paginação. |
| order | string | query | No | Direção de ordenação no horário do evento. Padrão DESC (mais recentes primeiro). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Retornar apenas eventos realizados por este nome de usuário. |
| ip | string | query | No | Retornar apenas eventos deste endereço IP. |
| crudType | string | query | No | Retornar apenas eventos deste tipo: c, r, u, d ou login. |
| resourceName | string | query | No | Retornar apenas eventos para este recurso, por exemplo Usuário ou Moderador. |
| targetId | string | query | No | Retornar apenas eventos cujo objeto afetado tem este ID. |
| target | string | query | No | Correspondência de substring sem distinção entre maiúsculas e minúsculas no rótulo do objeto afetado, por exemplo um nome de usuário ou e‑mail. |
| includeManagedTenants | boolean | query | No | Também retornar eventos de tenants que este tenant gerencia. |

## Resposta

Retorna: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // String | O tenant para ler os logs de auditoria.
    Integer limit = 56; // Integer | Quantos logs retornar. Máximo 10000, padrão 1000.
    Integer skip = 56; // Integer | Quantos logs pular, para paginação.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Direção de ordenação no horário do evento. Padrão DESC (mais recentes primeiro).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Retornar apenas eventos realizados por este nome de usuário.
    String ip = "ip_example"; // String | Retornar apenas eventos deste endereço IP.
    String crudType = "crudType_example"; // String | Retornar apenas eventos deste tipo: c, r, u, d ou login.
    String resourceName = "resourceName_example"; // String | Retornar apenas eventos para este recurso, por exemplo Usuário ou Moderador.
    String targetId = "targetId_example"; // String | Retornar apenas eventos cujo objeto afetado tem este ID.
    String target = "target_example"; // String | Correspondência de substring sem distinção entre maiúsculas e minúsculas no rótulo do objeto afetado, por exemplo um nome de usuário ou e‑mail.
    Boolean includeManagedTenants = true; // Boolean | Também retornar eventos de tenants que este tenant gerencia.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]