## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak | Tenant, dla którego odczytywane są logi audytu. |
| limit | integer | query | Nie | Ile logów zwrócić. Maksymalnie 10000, domyślnie 1000. |
| skip | integer | query | Nie | Ile logów pominąć, przy stronicowaniu. |
| order | string | query | Nie | Kierunek sortowania według czasu zdarzenia. Domyślnie DESC (najnowsze najpierw). |
| after | integer | query | Nie |  |
| before | integer | query | Nie |  |
| username | string | query | Nie | Zwróć tylko zdarzenia wykonane przez tę nazwę użytkownika. |
| ip | string | query | Nie | Zwróć tylko zdarzenia z tego adresu IP. |
| crudType | string | query | Nie | Zwróć tylko zdarzenia tego typu: c, r, u, d lub login. |
| resourceName | string | query | Nie | Zwróć tylko zdarzenia dla tego zasobu, np. User lub Moderator. |
| targetId | string | query | Nie | Zwróć tylko zdarzenia, których dotknięty obiekt ma ten identyfikator. |
| target | string | query | Nie | Niezależne od wielkości liter dopasowanie podciągu w etykiecie dotkniętego obiektu, np. nazwa użytkownika lub e‑mail. |
| includeManagedTenants | boolean | query | Nie | Zwróć także zdarzenia z tenantów zarządzanych przez tego tenanta. |

## Odpowiedź

Zwraca: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importuj klasy:
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
    
    // Skonfiguruj autoryzację klucza API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. "Token" (domyślnie null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Tenant, dla którego odczytywane są logi audytu.
    Integer limit = 56; // Integer | Ile logów zwrócić. Maksymalnie 10000, domyślnie 1000.
    Integer skip = 56; // Integer | Ile logów pominąć, przy stronicowaniu.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Kierunek sortowania według czasu zdarzenia. Domyślnie DESC (najnowsze najpierw).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Zwróć tylko zdarzenia wykonane przez tę nazwę użytkownika.
    String ip = "ip_example"; // String | Zwróć tylko zdarzenia z tego adresu IP.
    String crudType = "crudType_example"; // String | Zwróć tylko zdarzenia tego typu: c, r, u, d lub login.
    String resourceName = "resourceName_example"; // String | Zwróć tylko zdarzenia dla tego zasobu, np. User lub Moderator.
    String targetId = "targetId_example"; // String | Zwróć tylko zdarzenia, których dotknięty obiekt ma ten identyfikator.
    String target = "target_example"; // String | Niezależne od wielkości liter dopasowanie podciągu w etykiecie dotkniętego obiektu, np. nazwa użytkownika lub e‑mail.
    Boolean includeManagedTenants = true; // Boolean | Zwróć także zdarzenia z tenantów zarządzanych przez tego tenanta.
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

---