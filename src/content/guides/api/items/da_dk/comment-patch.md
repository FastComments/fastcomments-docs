[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dette API-endpoint giver mulighed for at opdatere en enkelt kommentar.

Bemærk:

- Dette API kan opdatere kommentar-widgetten "live", hvis ønsket (dette øger grundlæggende `creditsCost` fra `1` til `2`).
  - Dette kan gøre migrering af kommentarer mellem sider "live" (ændring af `urlId`).
  - Migrationer koster yderligere `2` credits, da siderne forudberegnes og dette er CPU-intensivt.
- I modsætning til create-API'et vil dette API IKKE automatisk oprette brugerobjekter i vores system, hvis e-mail er angivet.
- Kommentarer opdateret via dette API kan stadig kontrolleres for spam, hvis ønsket.
- Konfiguration som maks. kommentar-længde, hvis konfigureret via Customization Rule-adminsiden, gælder her.
- For at give brugere mulighed for at opdatere deres kommentartekst kan du blot angive `comment` i request body. Vi genererer den resulterende `commentHTML`.
  - Hvis du angiver både `comment` og `commentHTML`, vil vi ikke automatisk generere HTML'en.
  - Hvis brugeren tilføjer mentions eller hashtags i deres nye tekst, vil det stadig blive behandlet som i `POST`-API'et.
- Når du opdaterer `commenterEmail` på en kommentar, er det bedst også at angive `userId`. Ellers skal du sikre, at brugeren med denne e-mail tilhører din tenant, ellers vil anmodningen fejle.  
- Hvis den pågældende kommentar er låst (`isLocked: true`), afvises anmodningen med `code: 'locked'`. Lås kommentaren op først, opdater den, og lås den derefter igen, hvis ønsket.


[inline-code-attrs-start title = 'Minimalt eksempel på Comment PATCH cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for Comment PATCH-anmodning'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Brugeren der udfører opdateringen. Kan om ønsket bruges til at kontrollere, at de kan redigere kommentaren.  **/
    contextUserId?: string
	/** Skal vi tjekke, om den nye kommentar ligner spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Om kommentaren skal vises "live" for brugere, der ser forekomster af kommentar-widgetten med samme urlId. BEMÆRK: Fordobler kreditomkostningen fra 1 til 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for Comment PATCH-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Medtages ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Medtages ved fejl. **/
    reason?: string
}
[inline-code-end]