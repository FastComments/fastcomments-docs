### Χρήση Αυθεντικοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API στο ApiClient πριν κάνετε αιτήματα που απαιτούν αυθεντικοποίηση. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Δημιουργία και ρύθμιση του API client
        ApiClient apiClient = new ApiClient();

        // ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το κλειδί API σας (παίρνετέ το από το πίνακα ελέγχου του FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Δημιουργήστε το αντικείμενο API με τον ρυθμισμένο client
        DefaultApi api = new DefaultApi(apiClient);

        // Τώρα μπορείτε να κάνετε αιτήματα API με αυθεντικοποίηση
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
            // Συνήθη σφάλματα:
            // - 401: Το κλειδί API λείπει ή είναι άκυρο
            // - 400: Απέτυχε ο έλεγχος εγκυρότητας του αιτήματος
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

### Συνήθη Προβλήματα

1. **401 "missing-api-key" σφάλμα**: Βεβαιωθείτε ότι καλείτε `apiClient.setApiKey("YOUR_KEY")` πριν δημιουργήσετε το instance του DefaultApi.
2. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `DefaultApi` για server-side αιτήματα με αυθεντικοποίηση, `PublicApi` για client-side/δημόσια αιτήματα.
3. **Κενό κλειδί API (null)**: Το SDK θα παραλείψει αθόρυβα την αυθεντικοποίηση αν το κλειδί API είναι null, οδηγώντας σε σφάλματα 401.