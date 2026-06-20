Καταχώρηση σελίδων για έναν tenant. Χρησιμοποιείται από τον desktop client του FChat για να συμπληρώσει τη λίστα δωματίων του.
Απαιτείται το `enableFChat` να είναι true στην επιλυμένη προσαρμοσμένη ρύθμιση (custom config) για κάθε σελίδα.
Οι σελίδες που απαιτούν SSO φιλτράρονται με βάση την πρόσβαση ομάδας του χρήστη που κάνει το αίτημα.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| cursor | string | query | Όχι | Αδιαφανής δείκτης σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με το ίδιο `sortBy`. |
| limit | integer | query | Όχι | 1..200, προεπιλογή 50 |
| q | string | query | Όχι | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων. |
| sortBy | string | query | Όχι | Τρόπος ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητική). |
| hasComments | boolean | query | Όχι | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο. |

## Απόκριση

Επιστρέφει: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Εισαγωγή κλάσεων:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String cursor = "cursor_example"; // String | Αδιαφανής δείκτης σελιδοποίησης που επιστρέφεται ως `nextCursor` από προηγούμενο αίτημα. Συνδεδεμένος με το ίδιο `sortBy`.
    Integer limit = 56; // Integer | 1..200, προεπιλογή 50
    String q = "q_example"; // String | Προαιρετικό φίλτρο προθέματος τίτλου χωρίς διάκριση πεζών/κεφαλαίων.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Τρόπος ταξινόμησης. `updatedAt` (προεπιλογή, νεότερα πρώτα), `commentCount` (πρώτα τα περισσότερα σχόλια), ή `title` (αλφαβητική).
    Boolean hasComments = true; // Boolean | Αν true, επιστρέφονται μόνο σελίδες με τουλάχιστον ένα σχόλιο.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]