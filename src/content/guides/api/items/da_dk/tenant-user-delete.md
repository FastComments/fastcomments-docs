[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Denne rute giver mulighed for fjernelse af en `TenantUser` efter id.

Sletning af brugerens kommentarer er mulig via `deleteComments`-forespørgselsparameteren. Bemærk at hvis dette er true:

1. Alle brugerens kommentarer vil blive slettet live.
2. Alle __underordnede__ (nu forældreløse) kommentarer vil blive slettet eller anonymiseret baseret på hver kommentars tilknyttede sidekonfiguration. For eksempel, hvis trådsletningsmode er "anonymiser", vil svar forblive, og brugerens kommentarer vil blive anonymiseret. Dette gælder kun når `commentDeleteMode` er `Remove` (standardværdien).
3. `creditsCost` bliver `2`.

### Anonymiserede Kommentarer

Du kan bevare brugerens kommentarer, men blot anonymisere dem ved at sætte `commentDeleteMode=1`.

Hvis brugerens kommentarer anonymiseres, sættes følgende værdier til null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` og `isDeletedUser` sættes til `true`.

Ved rendering vil kommentar-widget'en bruge `DELETED_USER_PLACEHOLDER` (standard: "[deleted]") for brugerens navn og `DELETED_CONTENT_PLACEHOLDER` for kommentaren. Disse kan tilpasses via Widget-tilpasnings-UI'et.

### Eksempler

[inline-code-attrs-start title = 'TenantUser Fjernelse cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Fjernelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // default
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** You can set this to true to also delete the user's comments. This will double the credit cost. **/
    deleteComments?: 'true' | 'false'
    /** You can set this as desired to determine how to handle the user's comments. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantUser Fjernelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
