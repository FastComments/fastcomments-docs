[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ta pot omogoča odstranitev posameznega uporabnika SSO po njegovem id.

Upoštevajte, da bo ponovno nalaganje pripomočka za komentarje s podatki za tega uporabnika preprosto brezhibno ponovno ustvarilo uporabnika.

Brisanje komentarjev uporabnika je mogoče prek parametra poizvedbe `deleteComments`. Upoštevajte, da je ta `true`:

1. Vsi komentarji uporabnika bodo izbrisani v živo.
2. Vsi __podrejeni__ (zdaj osiroteli) komentarji bodo izbrisani ali anonimizirani glede na konfiguracijo strani, povezano s posameznim komentarjem. Na primer, če je način brisanja niti "anonymize", bodo odgovori ostali, komentarji uporabnika pa bodo anonimizirani. To velja samo, ko je `commentDeleteMode` nastavljen na `Remove` (privzeta vrednost).
3. `creditsCost` postane `2`.

### Anonimizirani komentarji

Komentarje uporabnika lahko ohranite, vendar jih preprosto anonimizirate, tako da nastavite `commentDeleteMode=1`.

Če so komentarji uporabnika anonimizirani, so naslednje vrednosti nastavljene na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` in `isDeletedUser` sta nastavljena na `true`.

Pri prikazu bo pripomoček za komentarje uporabil `DELETED_USER_PLACEHOLDER` (privzeto: "[deleted]") za ime uporabnika in `DELETED_CONTENT_PLACEHOLDER` za komentar. Te vrednosti je mogoče prilagoditi prek uporabniškega vmesnika za prilagoditve pripomočka.

### Primeri

[inline-code-attrs-start title = 'Primer cURL zahteve za odstranitev SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za odstranitev SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za odstranitev SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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