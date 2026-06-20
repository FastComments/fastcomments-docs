### Korzystanie z uwierzytelnionych interfejsów API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API w ApiClient przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Utwórz i skonfiguruj klienta API
        ApiClient apiClient = new ApiClient();

        // WYMAGANE: Ustaw swój klucz API (pobierz go z pulpitu FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Utwórz instancję API z skonfigurowanym klientem
        DefaultApi api = new DefaultApi(apiClient);

        // Teraz możesz wykonywać uwierzytelnione wywołania API
        try {
            // Przykład: Dodaj użytkownika SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Typowe błędy:
            // - 401: brak klucza API lub jest nieprawidłowy
            // - 400: walidacja żądania nie powiodła się
        }
    }
}
```

### Korzystanie z publicznych API (PublicApi)

Publiczne endpointy nie wymagają uwierzytelnienia:

```java
import com.fastcomments.api.PublicApi;
import com.fastcomments.invoker.ApiException;

PublicApi publicApi = new PublicApi();

try {
    var response = publicApi.getCommentsPublic("YOUR_TENANT_ID", "page-url-id")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Korzystanie z API moderacji (ModerationApi)

`ModerationApi` napędza panel moderatora. Każda metoda akceptuje parametr `sso`, identyfikujący moderatora uwierzytelnionego przez SSO, w imieniu którego wykonywane jest żądanie:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Wyświetl komentarze oczekujące na moderację
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Typowe problemy

1. **401 "missing-api-key" error**: Upewnij się, że wywołujesz `apiClient.setApiKey("YOUR_KEY")` przed utworzeniem instancji DefaultApi.
2. **Wrong API class**: Używaj `DefaultApi` do uwierzytelnionych żądań po stronie serwera, `PublicApi` do żądań po stronie klienta/publicznych.
3. **Null API key**: SDK pominie uwierzytelnianie bez komunikatu, jeśli klucz API jest null, co prowadzi do błędów 401.