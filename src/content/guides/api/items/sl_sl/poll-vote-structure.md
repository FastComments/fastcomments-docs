A `PollVote` je odgovor ene osebe na anketo. Števci, prikazani na sami anketi, so usklajeni s temi, zato jih potrebujete le, ko želite vedeti *kdo* je glasoval za kaj, namesto skupnih rezultatov.

Glasovalec ima največ en glas na anketo. Ponovno glasovanje premakne njihov obstoječi glas na novo možnost namesto, da bi dodal drugi glas, in `updatedAt` beleži, kdaj se je to zgodilo.

`voterId` je `userId`, ko je bil glasovalec prijavljen, sicer je `anonUserId`.

[inline-code-attrs-start title = 'Struktura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** userId, ko je bil glasovalec prijavljen, sicer anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Kdaj je glasovalec zadnjič premaknil svoj glas na drugo možnost. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

Nastavitev `privacy` ankete velja za ta API na enak način kot v pripomočku za komentarje:

- **Anonymous** (privzeto): nihče ne more videti, kako je kdo glasoval, zato glasov ni mogoče prebrati.
  `GET /api/v1/poll-votes` in `GET /api/v1/poll-votes/:id` odgovarjata z `poll-anonymous`. Števci ankete so še vedno na voljo prek `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: vaš API ključ pripada skrbniku vašega spletnega mesta, zato lahko bere glasove.
- **Everyone**: glasove je mogoče prebrati.

Zasebnost ankete je mogoče zožiti, vendar ne razširiti, ko ima glasove.