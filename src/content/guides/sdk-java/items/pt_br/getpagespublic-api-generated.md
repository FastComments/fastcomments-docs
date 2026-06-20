Lista páginas para um tenant. Usado pelo cliente desktop FChat para preencher sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.
Páginas que requerem SSO são filtradas com base no acesso de grupo do usuário solicitante.

## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Cursor de paginação opaco retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | No | 1..200, padrão 50 |
| q | string | query | No | Filtro opcional de prefixo de título sem diferenciar maiúsculas/minúsculas. |
| sortBy | string | query | No | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética). |
| hasComments | boolean | query | No | Se true, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
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
    String cursor = "cursor_example"; // String | Cursor de paginação opaco retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`.
    Integer limit = 56; // Integer | 1..200, padrão 50
    String q = "q_example"; // String | Filtro opcional de prefixo de título sem diferenciar maiúsculas/minúsculas.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentários primeiro), ou `title` (alfabética).
    Boolean hasComments = true; // Boolean | Se true, retorna apenas páginas com pelo menos um comentário.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]