A `PollVote` је одговор једне особе на анкету. Бројеви приказани на самој анкети се ажурирају у складу са овим,
па вам ово треба само када желите да знате *ко* је гласао за шта, а не укупне резултате.

Гласач има највише један глас по анкети. Поновно гласање премешта постојећи глас на нову опцију уместо
да додаје други, а `updatedAt` бележи када се то десило.

`voterId` је `userId` када је гласач пријављен, а иначе је `anonUserId`.

[inline-code-attrs-start title = 'Struktura PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** userId када је гласач пријављен, иначе anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** Када је гласач последњи пут преместио свој глас на другу опцију. **/
    updatedAt?: string
}
[inline-code-end]

### Приватност

Подешавање `privacy` анкете се примењује на овај API на исти начин као и у виџету за коментаре:

- **Anonymous** (подразумевано): нико не може видети како је неко гласао, па гласови не могу бити прочитани.
  `GET /api/v1/poll-votes` и `GET /api/v1/poll-votes/:id` одговарају са `poll-anonymous`. Бројеви анкете
  су и даље доступни преко `GET /api/v1/polls/:commentId`.
- **Admins and moderators**: ваш API кључ припада администратору вашег сајта, па може читати гласове.
- **Everyone**: гласови се могу читати.

Приватност анкете може бити сужена, али не и проширена након што има гласова.

---