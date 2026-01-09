[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om een gebruiker te blokkeren die een bepaalde opmerking heeft geschreven. Het ondersteunt blokkeren van opmerkingen geschreven door FastComments.com-gebruikers, SSO-gebruikers en tenantgebruikers.

Het ondersteunt een body-parameter `commentIdsToCheck` om te controleren of andere mogelijk zichtbare opmerkingen op de client geblokkeerd of gedeblokkeerd moeten worden nadat deze actie is uitgevoerd.

Notities:

- Deze oproep moet altijd worden uitgevoerd in de context van een gebruiker. De gebruiker kan een FastComments.com-gebruiker, een SSO-gebruiker of een tenantgebruiker zijn.
- Het `userId` in het verzoek is de gebruiker die de *blokkering uitvoert*. Bijvoorbeeld: `User A` wil `User B` blokkeren. Geef `userId=User A` en de comment-id die `User B` schreef.
- Volledig anonieme opmerkingen (geen user id, geen e-mail) kunnen niet worden geblokkeerd en er wordt een fout geretourneerd.

[inline-code-attrs-start title = 'cURL-voorbeeld: Opmerking blokkeren'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Voor anonieme blokkering moeten we een `anonUserId` opgeven. Dit kan een ID zijn die de anonieme sessie vertegenwoordigt, of een willekeurige UUID.
Dit stelt ons in staat om opmerkingen te blokkeren, zelfs als een gebruiker niet is ingelogd, door de opmerkingen op te halen met hetzelfde `anonUserId`.

[inline-code-attrs-start title = 'cURL-voorbeeld: Anonieme opmerking blokkeren'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Aanvraagstructuur: Opmerking blokkeren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Reactiestructuur: Opmerking blokkeren'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Opgenomen bij mislukking. **/
    reason?: string
    /** Als commentIdsToCheck gedefinieerd is, vermeldingen in deze map met de waarde true zijn ook geblokkeerd. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---