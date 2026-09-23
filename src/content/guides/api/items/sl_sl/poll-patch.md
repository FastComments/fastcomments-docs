[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Uredi anketo, ne da bi motil glasove. Uporabite to za popravitev tipkarske napake v vprašanju ali možnosti, za zapiranje ali ponovno odpiranje ankete, ali za spremembo, kdo lahko vidi, kdo je glasoval.

Možnosti so naslovljene po njihovem `id`, in `PATCH` preimenuje tiste, ki jih navedete. Za dodajanje, odstranjevanje ali preurejanje možnosti pošljite celoten seznam možnosti na `PUT /api/v1/polls/:commentId`: možnosti, ki jih pošljete s svojimi id-ji, ohranijo tudi svoje glasove.

Vsako polje je neobvezno, vendar mora biti podano vsaj eno.

[inline-code-attrs-start title = 'Primer cURL zahteve za popravilo ankete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteve za takojšnje zapiranje ankete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za popravilo ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Preimenuje obstoječe možnosti. Vsak podani id mora že biti v anketi. **/
    options?: { id: string, label: string }[] | null
    /** Datum v preteklosti takoj zapre anketo. null ponovno odpre zaprto anketo. **/
    closesAt?: string | null
    /** 0 anonimno, 1 administratorji in moderatorji, 2 vsi. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za popravilo ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Vključeno ob napaki. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- Poimenovanje id-ja možnosti, ki ni v anketi, povzroči napako `poll-invalid`, namesto tihega neukrepanja.
- Oznake morajo ostati edinstvene znotraj ankete, pri čemer se upoštevajo tudi možnosti, ki jih ne spreminjate.
- Za razliko od ustvarjanja ankete, je tukaj `closesAt` lahko v preteklosti – tako takoj zaprete anketo.
- Zasebnost ankete je mogoče zožiti, vendar ne razširiti, ko ima glasove.
- Zaklenjen komentar ne more imeti spremenjene ankete in povzroči napako `locked`.