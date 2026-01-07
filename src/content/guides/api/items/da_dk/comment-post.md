[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at oprette en kommentar.

De krav og begrænsninger, der gælder for at efterlade en kommentar via kommentar-widget'en, gælder også her. For eksempel, hvis du har aktiveret meddelelser om samtykke, og brugeren ikke har accepteret dem, vil de ikke modtage e-mail-meddelelser.

Denne rute adlyder også tenant-niveau indstillinger som `commentsRequireApproval`. Dette kan styres for denne anmodning via `isApproved`-flaget på anmodningen.

[inline-code-attrs-start title = 'Comment POST cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "urlId": "some-page-or-article",
    "url": "https://example.com/some-page-url",
    "pageTitle": "Some Page Title",
    "userId": "some-user-id",
    "commenterEmail": "someone@somewhere.com",
    "commenterName": "Some Name",
    "comment": "The **raw** comment text!"
}'
[inline-code-end]

Ved at bruge `userId` vil vi kæde denne kommentar til en eksisterende SSO-bruger. Hvis brugeren ikke eksisterer, vil de blive oprettet.

I dette tilfælde er `commenterEmail` og `commenterName` valgfrie. Hvis de ikke angives, vil de blive sat til de eksisterende værdier for brugeren. Hvis de
angives, vil de blive brugt i stedet.

Feltet `comment` er den rå, markdown-formaterede kommentar, maksimalt 16k tegn.

Følgende felter understøttes også ved oprettelse af en kommentar:

[inline-code-attrs-start title = 'Comment Oprettelsesfelter'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostBody {
    anonUserId?: string
    urlId: string
    url: string
    pageTitle?: string
    userId?: string
    commenterEmail?: string
    commenterName?: string
    comment: string
    parentId?: string | null
    date?: string
    /** Default is true. **/
    verified?: boolean
    avatarSrc?: string
    notificationLocaleOverride?: string
    /** Used with SSO to store any id you want. Max length 200 characters. Can be used with GET Comment APIs to filter by externalId. **/
    externalId?: string
    isSpam?: boolean
    isApproved?: boolean
    /** Defaults to false. Set to true to not send email notification for this comment, for example when bulk importing. **/
    skipNotifications?: boolean
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment POST Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-comment' | 'invalid-url' | 'invalid-url-id' | 'invalid-page-title' | 'invalid-parent-id' | 'invalid-date' | 'invalid-verified' | 'invalid-avatar-src' | 'empty-request' | 'invalid-input'
    /** Included on failure. **/
    reason?: string
    comment?: Comment
}
[inline-code-end]

### Oprettelse af Svar

For at oprette et svar på en kommentar skal du angive `parentId`-feltet. Dette er ID'et på den kommentar, du svarer på.

[inline-code-attrs-start title = 'Reply Comment POST cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "urlId": "test",
    "url": "https://example.com/test",
    "pageTitle": "Test",
    "userId": "some-user-id",
    "commenterEmail": "someone@somewhere.com",
    "commenterName": "Some Name",
    "comment": "En **svar**!",
    "parentId": "some-parent-comment-id"
}'
[inline-code-end]

### Live API vs Doven Tilstand

Som standard er denne API "doven" - den sender ikke meddelelser eller opdaterer kommentarer i realtid. Dette er nyttigt til migrationer. For at aktivere live-tilstand skal du sætte `skipNotifications` til `false`.
