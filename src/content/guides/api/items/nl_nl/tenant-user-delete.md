[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Deze route maakt het mogelijk om een `TenantUser` te verwijderen op basis van id.

Het is mogelijk om de opmerkingen van de gebruiker te verwijderen via de queryparameter `deleteComments`. Let op dat wanneer dit waar is:

1. Alle opmerkingen van de gebruiker worden direct verwijderd.
2. Alle __child__ (nu wees) opmerkingen worden verwijderd of geanonimiseerd op basis van de pagina-configuratie die aan elke opmerking is gekoppeld. Bijvoorbeeld, als thread deletion mode "anonymize" is, blijven reacties behouden en worden de opmerkingen van de gebruiker geanonimiseerd. Dit geldt alleen wanneer `commentDeleteMode` `Remove` is (de standaardwaarde).
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

`isDeleted` en `isDeletedUser` worden ingesteld op `true`.

Bij weergave zal de comment-widget `DELETED_USER_PLACEHOLDER` (standaard: "[deleted]") gebruiken voor de naam van de gebruiker en `DELETED_CONTENT_PLACEHOLDER` voor de opmerking. Deze kunnen worden aangepast via de Widget Customization UI.

### Voorbeelden

[inline-code-attrs-start title = 'cURL-voorbeeld voor TenantUser-verwijdering'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van TenantUser-verwijderverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // standaard
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** U kunt dit op true zetten om ook de opmerkingen van de gebruiker te verwijderen. Dit verdubbelt de kosten in credits. **/
    deleteComments?: 'true' | 'false'
    /** U kunt dit instellen om te bepalen hoe met de opmerkingen van de gebruiker om te gaan. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van TenantUser-verwijderingsantwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij falen. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Opgenomen bij falen. **/
    reason?: string
}
[inline-code-end]