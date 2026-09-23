[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Uređuje anketu bez ometanja njenih glasova. Koristite ovo za ispravljanje tipfelera u pitanju ili opciji, za zatvaranje ili ponovno otvaranje ankete, ili za promjenu tko može vidjeti tko je glasao.

Opcije se adresiraju prema njihovom `id`, a `PATCH` preimenuje one koje navedete. Za dodavanje, uklanjanje ili promjenu redoslijeda opcija, pošaljite cijeli popis opcija na `PUT /api/v1/polls/:commentId`: opcije koje pošaljete s njihovim id-ovima zadržavaju i svoje glasove.

Sva polja su opcionalna, ali mora biti navedeno barem jedno.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za ažuriranje ankete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura zahtjeva za ažuriranje ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora na ažuriranje ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Other Notes

- Navođenje id-a opcije koji nije u anketi rezultira greškom `poll-invalid` umjesto tihe neaktivnosti.
- Oznake moraju ostati jedinstvene unutar ankete, uzimajući u obzir opcije koje ne mijenjate.
- Za razliku od stvaranja ankete, `closesAt` ovdje može biti u prošlosti – to je način kako odmah zatvoriti anketu.
- Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasova.
- Zaključani komentar ne može imati promijenjenu anketu i rezultira greškom `locked`.