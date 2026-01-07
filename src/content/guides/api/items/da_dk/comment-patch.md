[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at opdatere en enkelt kommentar.

Bemærkninger:

- Denne API kan opdatere kommentar-widget'en "live" hvis ønsket (dette øger basis-`creditsCost` fra `1` til `2`).
  - Dette kan gøre migrering af kommentarer mellem sider "live" (ændring af `urlId`).
  - Migreringer koster yderligere `2` kreditter, da sider er forudberegnet, og dette er CPU-intensivt.
- I modsætning til oprettelses-API'et vil denne API IKKE automatisk oprette brugerobjekter i vores system, hvis e-mail er angivet.
- Kommentarer opdateret via denne API kan stadig kontrolleres for spam, hvis ønsket.
- Konfiguration såsom maksimal kommentarlængde, hvis konfigureret via Tilpasningsregel-administrationssiden, vil gælde her.
- For at tillade brugere at opdatere deres kommentartekst, kan du bare angive `comment` i anmodningskroppen. Vi vil generere den resulterende `commentHTML`.
  - Hvis du definerer både `comment` og `commentHTML`, vil vi ikke automatisk generere HTML'en.
  - Hvis brugeren tilføjer omtaler eller hashtags i deres nye tekst, vil det stadig blive behandlet som `POST` API'et.
- Når du opdaterer `commenterEmail` på en kommentar, er det bedst også at angive `userId`. Ellers skal du sikre, at brugeren med denne e-mail tilhører din tenant, ellers vil anmodningen fejle.


[inline-code-attrs-start title = 'Minimum Comment PATCH cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Comment PATCH Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment PATCH Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
