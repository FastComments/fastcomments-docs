## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Ο ενοικιαστής για τον οποίο διαβάζονται τα αρχεία ελέγχου. |
| limit | integer | query | No | Πόσα αρχεία καταγραφής να επιστραφούν. Μέγιστο 10000, προεπιλογή 1000. |
| skip | integer | query | No | Πόσα αρχεία καταγραφής να παραλειφθούν, για σελιδοποίηση. |
| order | string | query | No | Κατεύθυνση ταξινόμησης του χρόνου συμβάντος. Προεπιλογή DESC (πρώτα τα πιο πρόσφατα). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Επιστρέφει μόνο τα συμβάντα που εκτελέστηκαν από αυτό το όνομα χρήστη. |
| ip | string | query | No | Επιστρέφει μόνο τα συμβάντα από αυτή τη διεύθυνση IP. |
| crudType | string | query | No | Επιστρέφει μόνο τα συμβάντα αυτού του τύπου: c, r, u, d ή login. |
| resourceName | string | query | No | Επιστρέφει μόνο τα συμβάντα για αυτόν τον πόρο, π.χ. Χρήστη ή Συντονιστή. |
| targetId | string | query | No | Επιστρέφει μόνο τα συμβάντα του αντικειμένου που επηρεάζεται με αυτό το αναγνωριστικό. |
| target | string | query | No | Ανίχνευση υποσυμβολοσειράς χωρίς διάκριση πεζών-κεφαλαίων στην ετικέτα του επηρεαζόμενου αντικειμένου, π.χ. όνομα χρήστη ή email. |
| includeManagedTenants | boolean | query | No | Επιστρέφει επίσης συμβάντα από ενοικιαστές που διαχειρίζεται αυτός ο ενοικιαστής. |

## Απάντηση

Επιστρέφει: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Εισαγωγή κλάσεων:
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
    
    // Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε πρόθεμα για το κλειδί API, π.χ. "Token" (προεπιλογή null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Ο ενοικιαστής για τον οποίο διαβάζονται τα αρχεία ελέγχου.
    Integer limit = 56; // Integer | Πόσα αρχεία καταγραφής να επιστραφούν. Μέγιστο 10000, προεπιλογή 1000.
    Integer skip = 56; // Integer | Πόσα αρχεία καταγραφής να παραλειφθούν, για σελιδοποίηση.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Κατεύθυνση ταξινόμησης του χρόνου συμβάντος. Προεπιλογή DESC (πρώτα τα πιο πρόσφατα).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Επιστρέφει μόνο τα συμβάντα που εκτελέστηκαν από αυτό το όνομα χρήστη.
    String ip = "ip_example"; // String | Επιστρέφει μόνο τα συμβάντα από αυτή τη διεύθυνση IP.
    String crudType = "crudType_example"; // String | Επιστρέφει μόνο τα συμβάντα αυτού του τύπου: c, r, u, d ή login.
    String resourceName = "resourceName_example"; // String | Επιστρέφει μόνο τα συμβάντα για αυτόν τον πόρο, π.χ. Χρήστη ή Συντονιστή.
    String targetId = "targetId_example"; // String | Επιστρέφει μόνο τα συμβάντα του αντικειμένου που επηρεάζεται με αυτό το αναγνωριστικό.
    String target = "target_example"; // String | Ανίχνευση υποσυμβολοσειράς χωρίς διάκριση πεζών-κεφαλαίων στην ετικέτα του επηρεαζόμενου αντικειμένου, π.χ. όνομα χρήστη ή email.
    Boolean includeManagedTenants = true; // Boolean | Επιστρέφει επίσης συμβάντα από ενοικιαστές που διαχειρίζεται αυτός ο ενοικιαστής.
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