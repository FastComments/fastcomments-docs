[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at fjerne en enkelt SSO-bruger efter deres id.

Bemærk at indlæsning af kommentar-widget'en igen med en payload for denne bruger simpelthen vil genskabe brugeren problemfrit.

Sletning af brugerens kommentarer er mulig via `deleteComments` query-parameteren. Bemærk at hvis dette er sandt:

1. Alle brugerens kommentarer vil blive slettet live.
2. Alle __underordnede__ (nu forældreløse) kommentarer vil blive slettet eller anonymiseret baseret på hver kommentars tilknyttede sidekonfiguration. For eksempel, hvis trådsletningstilstand er "anonymize", så forbliver svar, og brugerens kommentarer vil blive anonymiseret. Dette gælder kun, når `commentDeleteMode` er `Remove` (standardværdien).
3. `creditsCost` bliver `2`.

### Anonymiserede Kommentarer

Du kan beholde brugerens kommentarer, men blot anonymisere dem ved at sætte `commentDeleteMode=1`.

Hvis brugerens kommentarer er anonymiseret, så sættes følgende værdier til null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` og `isDeletedUser` sættes til `true`.

Ved rendering vil kommentar-widget'en bruge `DELETED_USER_PLACEHOLDER` (standard: "[deleted]") for brugerens navn og `DELETED_CONTENT_PLACEHOLDER` for kommentaren. Disse kan tilpasses via Widget Customization UI.

### Eksempler

[inline-code-attrs-start title = 'SSOUser Fjernelse cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Fjernelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser Fjernelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the removed user on success.
}
[inline-code-end]
