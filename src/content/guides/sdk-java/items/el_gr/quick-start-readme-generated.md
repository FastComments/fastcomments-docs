### Χρήση Αυθεντικοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το API key σας στον ApiClient πριν κάνετε αιτήματα που απαιτούν αυθεντικοποίηση. Εάν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Δημιουργήστε και διαμορφώστε τον API client
        ApiClient apiClient = new ApiClient();

        // ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το API key σας (λάβετε το από το dashboard του FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Δημιουργήστε το API instance με τον διαμορφωμένο client
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
            // Συνηθισμένα σφάλματα:
            // - 401: Το API key λείπει ή είναι άκυρο
            // - 400: Η επικύρωση του αιτήματος απέτυχε
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

### Χρήση API Εποπτείας (ModerationApi)

Το `ModerationApi` χειρίζεται τον πίνακα ελέγχου του συντονιστή. Κάθε μέθοδος δέχεται μια παράμετρο `sso` που προσδιορίζει τον συντονιστή αυθεντικοποιημένο μέσω SSO για λογαριασμό του οποίου γίνεται το αίτημα:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Λίστα σχολίων που περιμένουν έλεγχο/εποπτεία
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Συνήθη Προβλήματα

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι καλείτε `apiClient.setApiKey("YOUR_KEY")` πριν δημιουργήσετε το instance του DefaultApi.
2. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για αιτήματα διακομιστή με αυθεντικοποίηση, `PublicApi` για client-side/δημόσια αιτήματα.
3. **Μηδενικό API key (null)**: Το SDK θα παραλείψει σιωπηλά την αυθεντικοποίηση αν το API key είναι null, οδηγώντας σε σφάλματα 401.