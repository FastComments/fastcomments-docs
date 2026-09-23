[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Uređuje anketu bez ometanja njenih glasova. Koristite ovo da ispravite tipografsku grešku u pitanju ili opciji, da zatvorite ili ponovo otvorite anketu, ili da promenite ko može da vidi ko je glasao.

Opcije se adresiraju po njihovom `id`, a `PATCH` preimenuje one koje navedete. Da biste dodali, uklonili ili promenili redosled opcija, pošaljite kompletnu listu opcija na `PUT /api/v1/polls/:commentId`: opcije koje pošaljete sa njihovim id-jevima zadržavaju i svoje glasove.

Svako polje je opciono, ali mora biti navedeno bar jedno.

[inline-code-attrs-start title = 'Primer cURL zahteva za ažuriranje ankete'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteva za zatvaranje ankete odmah'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za ažuriranje ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Preimenuje postojeće opcije. Svaki prosleđeni id mora već biti u anketi. **/
    options?: { id: string, label: string }[] | null
    /** Datum u prošlosti zatvara anketu odmah. null ponovo otvara zatvorenu anketu. **/
    closesAt?: string | null
    /** 0 anonimno, 1 administratori i moderatori, 2 svi. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za ažuriranje ankete'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Uključeno u slučaju greške. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Ostale napomene

- Davanje id-a opciji koja nije u anketi rezultuje greškom `poll-invalid` umesto tihe neaktivnosti.
- Oznake moraju ostati jedinstvene unutar ankete, uzimajući u obzir i opcije koje ne menjate.
- Za razliku od kreiranja ankete, `closesAt` ovde može biti u prošlosti – tako odmah zatvarate anketu.
- Privatnost ankete može se suziti, ali ne i proširiti nakon što ima glasova.
- Zaključani komentar ne može imati izmenjenu anketu i rezultuje greškom `locked`.