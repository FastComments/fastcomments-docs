Μεταφόρτωση και αλλαγή μεγέθους εικόνας

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| sizePreset | string | query | Όχι | Προεπιλογή μεγέθους: "Default" (1000x1000px) ή "CrossPlatform" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) |
| urlId | string | query | Όχι | ID της σελίδας από την οποία γίνεται η μεταφόρτωση, για διαμόρφωση |

## Απόκριση

Επιστρέφει: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UploadImageResponse.java)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα uploadImage'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    File _file = new File("/path/to/file"); // File | 
    SizePreset sizePreset = SizePreset.fromValue("Default"); // SizePreset | Προεπιλογή μεγέθους: \"Default\" (1000x1000px) ή \"CrossPlatform\" (δημιουργεί μεγέθη για δημοφιλείς συσκευές)
    String urlId = "urlId_example"; // String | ID της σελίδας από την οποία γίνεται η μεταφόρτωση, για διαμόρφωση
    try {
      UploadImageResponse result = apiInstance.uploadImage(tenantId, _file)
            .sizePreset(sizePreset)
            .urlId(urlId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#uploadImage");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]