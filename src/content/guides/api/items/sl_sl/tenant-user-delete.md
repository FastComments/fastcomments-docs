[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ta ruta omogoča odstranitev `TenantUser` po id.

Brisanje komentarjev uporabnika je možno preko poizvedbenega parametra `deleteComments`. Upoštevajte, da če je to true:

1. Vsi komentarji uporabnika bodo takoj izbrisani.
2. Vsi __child__ (sedaj osiroteli) komentarji bodo izbrisani ali anonimizirani glede na konfiguracijo strani, povezane z vsakim komentarjem. Na primer, če je thread deletion mode "anonymize", bodo odgovori ostali, komentatorjevi komentarji pa bodo anonimizirani. To velja samo, ko je `commentDeleteMode` nastavljeno na `Remove` (privzeta vrednost).
3. Vrednost `creditsCost` postane `2`.

### Anonimizirani komentarji

Komentarje uporabnika lahko obdržite, vendar jih preprosto anonimizirate tako, da nastavite `commentDeleteMode=1`.

Če so komentarji uporabnika anonimizirani, se naslednje vrednosti nastavijo na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` and `isDeletedUser` sta nastavljena na `true`.

Pri upodabljanju bo komentarni pripomoček za ime uporabnika uporabil `DELETED_USER_PLACEHOLDER` (privzeto: "[deleted]") in `DELETED_CONTENT_PLACEHOLDER` za vsebino komentarja. To je mogoče prilagoditi prek uporabniškega vmesnika za prilagajanje widgeta.

### Primeri

[inline-code-attrs-start title = 'Primer cURL za odstranitev TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za odstranitev TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // privzeto
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** To lahko nastavite na true, da prav tako izbrišete komentarje uporabnika. To bo podvojilo strošek kreditov. **/
    deleteComments?: 'true' | 'false'
    /** To lahko nastavite po želji, da določite, kako ravnati s komentarji uporabnika. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za odstranitev TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Vključeno ob neuspehu. **/
    reason?: string
}
[inline-code-end]