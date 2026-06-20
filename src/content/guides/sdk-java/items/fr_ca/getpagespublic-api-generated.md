Lister les pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Requiert que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent SSO sont filtrées en fonction de l'accès de groupe de l'utilisateur demandeur.

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| cursor | string | query | Non | Curseur de pagination opaque renvoyé en tant que `nextCursor` par une requête précédente. Lié au même `sortBy`. |
| limit | integer | query | Non | 1..200, par défaut 50 |
| q | string | query | Non | Filtre optionnel sur le préfixe du titre, insensible à la casse. |
| sortBy | string | query | Non | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (les plus commentés en premier), ou `title` (alphabétique). |
| hasComments | boolean | query | Non | Si vrai, ne retourner que les pages ayant au moins un commentaire. |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    String cursor = "cursor_example"; // String | Curseur de pagination opaque renvoyé en tant que `nextCursor` par une requête précédente. Lié au même `sortBy`.
    Integer limit = 56; // Integer | 1..200, par défaut 50
    String q = "q_example"; // String | Filtre optionnel sur le préfixe du titre, insensible à la casse.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Ordre de tri. `updatedAt` (par défaut, les plus récents en premier), `commentCount` (les plus commentés en premier), ou `title` (alphabétique).
    Boolean hasComments = true; // Boolean | Si vrai, ne retourner que les pages ayant au moins un commentaire.
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