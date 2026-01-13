### Korzystanie z uwierzytelnionych interfejsów API (DefaultApi)

**Ważne:** Musisz ustawić swój klucz API na ApiClient przed wykonywaniem uwierzytelnionych żądań. Jeśli tego nie zrobisz, żądania zakończą się błędem 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Create and configure the API client
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Create the API instance with the configured client
        DefaultApi api = new DefaultApi(apiClient);

        // Now you can make authenticated API calls
        try {
            // Example: Add an SSO user
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Common errors:
            // - 401: API key is missing or invalid
            // - 400: Request validation failed
        }
    }
}
```

### Korzystanie z publicznych interfejsów API (PublicApi)

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

### Częste problemy

1. **401 "missing-api-key" error**: Upewnij się, że wywołujesz `apiClient.setApiKey("YOUR_KEY")` przed utworzeniem instancji DefaultApi.
2. **Wrong API class**: Użyj `DefaultApi` do uwierzytelnionych żądań po stronie serwera, `PublicApi` do żądań po stronie klienta/publicznych.
3. **Null API key**: SDK automatycznie pominie uwierzytelnianie, jeśli klucz API ma wartość null, co spowoduje błędy 401.