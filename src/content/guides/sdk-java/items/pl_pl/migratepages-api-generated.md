Kolejkuje zadanie wsadowe, które przenosi komentarze, strony i powiązane rekordy z jednego urlId lub domeny do innego.  
"urlId" migracje dopasowują tylko dokładny urlId. "domain" migracje wyszukują i zamieniają wartość wewnątrz każdego urlId i url.  
Poll GET /api/v1/batch-jobs/{batchJobId}: po zakończeniu zwraca wyniki per‑migracji oraz undoId.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład migratePages'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Skonfiguruj autoryzację kluczem API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentuj poniższą linię, aby ustawić prefiks dla klucza API, np. "Token" (domyślnie null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    MigrateAPIPagesData migrateAPIPagesData = new MigrateAPIPagesData(); // MigrateAPIPagesData | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.migratePages(tenantId, migrateAPIPagesData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Wyjątek podczas wywoływania DefaultApi#migratePages
      System.err.println("Exception when calling DefaultApi#migratePages");
      // Kod statusu: " + e.getCode());
      System.err.println("Status code: " + e.getCode());
      // Powód: " + e.getResponseBody());
      System.err.println("Reason: " + e.getResponseBody());
      // Nagłówki odpowiedzi: " + e.getResponseHeaders());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]