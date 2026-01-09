### Χρήση Αυθεντικοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το API key σας στο ApiClient πριν κάνετε αιτήσεις που απαιτούν αυθεντικοποίηση. Εάν δεν το κάνετε, οι αιτήσεις θα αποτύχουν με σφάλμα 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Δημιουργία και ρύθμιση του API client
        ApiClient apiClient = new ApiClient();

        // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το API key σας (το πάρετε από τον πίνακα ελέγχου FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Δημιουργήστε το API instance με τον ρυθμισμένο client
        DefaultApi api = new DefaultApi(apiClient);

        // Τώρα μπορείτε να κάνετε ελεγμένες κλήσεις API
        try {
            // Παράδειγμα: Προσθήκη χρήστη SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Συνηθισμένα σφάλματα:
            // - 401: API key is missing or invalid
            // - 400: Request validation failed
        }
    }
}
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια endpoints δεν απαιτούν αυθεντικοποίηση:

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

### Συνηθισμένα Προβλήματα

1. **401 "missing-api-key" σφάλμα**: Βεβαιωθείτε ότι καλείτε `apiClient.setApiKey("YOUR_KEY")` πριν δημιουργήσετε το DefaultApi instance.
2. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `DefaultApi` για server-side αιτήσεις με αυθεντικοποίηση, `PublicApi` για client-side/δημόσιες αιτήσεις.
3. **Μηδενικό API key**: Το SDK θα παραλείψει σιωπηλά την αυθεντικοποίηση εάν το API key είναι null, οδηγώντας σε σφάλματα 401.