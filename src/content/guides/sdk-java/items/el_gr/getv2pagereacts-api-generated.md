## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Όταν οριστεί, επιστρέφει τους χρήστες για κάθε react id (εικονίδια και ονόματα χρηστών). Μόνο το \"public\" γίνεται αποδεκτό, το οποίο παραλείπει τα ιδιωτικά προφίλ. |

## Απάντηση

Επιστρέφει: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV2PageReacts.java)

## Παράδειγμα

[inline-code-attrs-start title = 'getV2PageReacts Παράδειγμα'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | Όταν οριστεί, επιστρέφει τους χρήστες για κάθε react id (εικονίδια και ονόματα χρηστών). Μόνο το \"public\" γίνεται αποδεκτό, το οποίο παραλείπει τα ιδιωτικά προφίλ.
    try {
      GetV2PageReacts result = apiInstance.getV2PageReacts(tenantId, urlId)
            .sso(sso)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getV2PageReacts");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]