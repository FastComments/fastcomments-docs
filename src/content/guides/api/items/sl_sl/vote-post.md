[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Ta pot omogoča dodajanje enega pooblaščenega `Vote`. Glasovi so lahko `up` (+1) ali `down` (-1).

[inline-code-attrs-start title = 'Primer cURL zahteve za ustvarjanje glasu'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteve za ustvarjanje anonimnega glasu'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za ustvarjanje glasu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ustvarjanje glasu'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Vključeno ob neuspehu. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Ustvarjanje anonimnih glasov

Anonimne glasove lahko ustvarite tako, da v parametrih poizvedbe nastavite `anonUserId` namesto `userId`.

Ta id ne rabi ustrezati nobenemu uporabniškemu objektu nikjer (od tod anonimno). To je preprosto identifikator
za sejo, tako da lahko v isti seji ponovno pridobite glasove, da preverite, ali je bil komentar glasovan.

Če nimate česa takega kot "anonymous sessions", kot jih ima FastComments - lahko preprosto
to nastavite na naključen ID, na primer UUID (čeprav cenimo krajše identifikatorje zaradi prihranka prostora).

### Druge opombe

- Ta API upošteva nastavitve na ravni najemnika. Na primer, če onemogočite glasovanje za določeno stran, in poskusite ustvariti glas preko API-ja, bo spodletelo s kodo napake `voting-disabled`.
- Ta API je privzeto aktiven.
- Ta API bo posodobil `votes` ustreznega `Comment`.