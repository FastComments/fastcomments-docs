**ИД шаблона:** `top_comment_pinner`

Pričvršćivač najboljih komentara nadzire komentare na првом нивоу који пређу праг гласова и причврšћује их - замењујући оно што је раније било причвршћено у истој нити.

### Ugrađeni početni prompt

[inline-code-attrs-start title = 'Početni prompt šablona Pričvršćivača najboljeg komentara'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

Instrukcija "do not pin replies" je важна: причвршћивање функционише по нитима, па је причвршћивање одговора ретко корисно. Филтер "non-promotional" спречава да агент појачава популаран коментар-спам са линковима.

### Okidači

- **Komentar pređe prag glasova** (`COMMENT_VOTE_THRESHOLD`, подразумевани праг гласова: 10).

Okидач се активира када нето гласови коментара (`up - down`) достигну конфигурисани праг. Подесите број на форми за уређивање у зависности од тога колико су ваше нити активне - 10 је разумна подразумевана вредност за умерено активне сајтове.

### Dozvoljeni alati

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

Причвршћивање није деструктивно - може се одмах поништити - зато овај шаблон обично ради без одобрења.

### Preporučeni dodaci pre puštanja u rad

- **Označite "Uključi roditeljski komentar i prethodne odgovore u istoj niti"** у [Opcije konteksta](#context-options). Без контекста нити агент не може поуздано утврдити постоји ли већ приквачен коментар који треба уклонити.
- **Подесите праг гласова** према вашем сајту. На заузетим нитима 10 се дешава превише често; на тихим нитима 10 можда никада неће бити достиђено.
- **Размотрите ограничавање по URL‑у** ако желите да приквачени коментари буду само у одређеним секцијама вашег сајта - нпр. вести, али не и нити за обавештења.

### Напомена о дуплом причвршћивању

Агентов prompt налаже да прво уклони постојеће приквачење пре него што причврсти нови коментар, али ако модел прескочи тај корак сама платформа не спроводи правило о једном прикваченом коментару по нити (може их бити више). Ако је дупло причвршћивање проблем на вашем сајту, ставите `pin_comment` под одобрење и прегледајте сваки случај - или напишите строжији prompt.

---