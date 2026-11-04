[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Uređuje anketu bez ometanja njenih glasova. Koristite ovo za ispravku greške u pitanju ili opciji, za zatvaranje ili
ponovno otvaranje ankete, ili za promenu toga ko može videti ko je glasao.

Opcije se adresiraju preko njihovog `id`, a `PATCH` preimenuje one koje navedete. Da biste dodali, uklonili ili
prerasporedili opcije, pošaljite kompletnu listu opcija na `PUT /api/v1/polls/:commentId`: opcije koje pošaljete sa
njihovim id-jevima zadržavaju svoje glasove i tamo.

Svako polje je opciono, ali barem jedno mora biti dato.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za Poll Patch'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za trenutno zatvaranje ankete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Poll Patch zahtjeva'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Poll Patch odgovora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Ostale napomene

- Navođenje id-ja opcije koja nije na anketi ne uspijeva s `poll-invalid` umjesto da tiho ne učini ništa.
- Oznake moraju ostati jedinstvene unutar ankete, računajući i opcije koje ne mijenjate.
- Za razliku od stvaranja ankete, `closesAt` ovdje može biti u prošlosti - tako se anketa zatvara odmah.
- Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasova.
- Zaključani komentar ne može imati promijenjenu anketu i ne uspijeva s `locked`.