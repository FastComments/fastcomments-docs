Remove a poll from its comment, along with every vote cast on it. The comment itself is left alone.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Example

[inline-code-attrs-start title = 'deletePoll Przykład'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.deletePoll(tenantId, commentId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Wyjątek podczas wywoływania DefaultApi#deletePoll");
      System.err.println("Kod statusu: " + e.getCode());
      System.err.println("Powód: " + e.getResponseBody());
      System.err.println("Nagłówki odpowiedzi: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---