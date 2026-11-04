[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Dit API-endpoint biedt de mogelijkheid om een gebruiker die een bepaalde reactie heeft geschreven te deblokkeren. Het ondersteunt het deblokkeren van reacties geschreven door FastComments.com Gebruikers, SSO Gebruikers en Tenant Gebruikers.

Het ondersteunt een `commentIdsToCheck` body-parameter om te controleren of andere mogelijk zichtbare reacties op de client geblokkeerd/gedeblokkeerd moeten worden nadat deze actie is uitgevoerd.

Opmerkingen:

- Deze aanroep moet altijd worden gedaan in de context van een gebruiker. De gebruiker kan een FastComments.com Gebruiker, SSO Gebruiker of Tenant Gebruiker zijn.
- De `userId` in het verzoek is de gebruiker die *het deblokkeren uitvoert*. Bijvoorbeeld: `Gebruiker A` wil `Gebruiker B` deblokkeren. Geef `userId=Gebruiker A` door en het id van de reactie die `Gebruiker B` heeft geschreven.
- Volledig anonieme reacties (geen gebruikers-id, geen e-mail) kunnen niet worden geblokkeerd en er wordt een foutmelding geretourneerd.

[inline-code-attrs-start title = 'Comment Deblokkeren cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonieme Reactie Deblokkeren cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Deblokkeren Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Deblokkeren Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Inbegrepen bij mislukking. **/
    reason?: string
    /** Als commentIdsToCheck is gedefinieerd, zijn items in deze map met true nog steeds geblokkeerd. Indien false, wil je de reacties mogelijk weer zichtbaar maken voor de gebruiker zodat deze niet hoeft te vernieuwen. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]