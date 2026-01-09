[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Deze route verzorgt het verwijderen van een enkele SSO-gebruiker op basis van hun id.

Let op: het opnieuw laden van de commentaarwidget met een payload voor deze gebruiker zal de gebruiker eenvoudig en naadloos opnieuw aanmaken.

Het verwijderen van de opmerkingen van de gebruiker is mogelijk via de queryparameter `deleteComments`. Let op: als dit true is:

1. Alle opmerkingen van de gebruiker worden live verwijderd.
2. Alle __child__ (nu verweesde) opmerkingen worden verwijderd of geanonimiseerd op basis van de paginaconfiguratie die aan elke opmerking is gekoppeld. Bijvoorbeeld, als de thread deletion mode "anonymize" is, blijven de antwoorden staan en worden de opmerkingen van de gebruiker geanonimiseerd. Dit geldt alleen wanneer `commentDeleteMode` `Remove` is (de standaardwaarde).
3. De `creditsCost` wordt `2`.

### Geanonimiseerde opmerkingen

U kunt de opmerkingen van de gebruiker behouden maar eenvoudigweg anonimiseren door `commentDeleteMode=1` in te stellen.

Als de opmerkingen van de gebruiker geanonimiseerd worden, worden de volgende waarden op null gezet:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` en `isDeletedUser` worden op `true` gezet.

Bij het renderen gebruikt de commentaarwidget `DELETED_USER_PLACEHOLDER` (standaard: "[deleted]") voor de naam van de gebruiker en `DELETED_CONTENT_PLACEHOLDER` voor de opmerking. Deze kunnen aangepast worden via de UI voor widgetaanpassing.

### Voorbeelden

[inline-code-attrs-start title = 'SSOUser Verwijdering cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van SSOUser Verwijderingsaanvraag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // standaard
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** U kunt dit op true zetten om ook de opmerkingen van de gebruiker te verwijderen. Dit verdubbelt de creditkosten. **/
    deleteComments?: 'true' | 'false'
    /** U kunt dit instellen om te bepalen hoe met de opmerkingen van de gebruiker wordt omgegaan. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van SSOUser Verwijderingsantwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Opgenomen bij een fout. **/
    reason?: string
    user?: SSOUser; // We geven de verwijderde gebruiker terug bij succes.
}
[inline-code-end]