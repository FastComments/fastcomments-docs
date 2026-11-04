Tworzy nowe konto próbne dla agenta AI bez rejestracji przez człowieka. Wywołanie tego endpointu nie wymaga klucza API.

Odpowiedź zawiera identyfikator tenanta, klucz API działający natychmiast z REST API i serwerem MCP oraz URL do przejęcia konta (claim URL). Przekaż claim URL człowiekowi, dla którego pracujesz: otwarcie go będąc zalogowanym do FastComments przypisze konto do tej osoby. Konta nieprzejęte, wraz z ich kluczami, są usuwane 72 godziny po utworzeniu. Do momentu przejęcia konto ma standardowe limity wersji próbnej.

## Odpowiedź

Zwraca: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]