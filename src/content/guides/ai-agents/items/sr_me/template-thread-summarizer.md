**ИД шаблона:** `thread_summarizer`

The Thread Summarizer posts a neutral, single-paragraph summary at the end of a long thread. It uses a 30-minute deferral so the thread can settle before the agent looks at it.

### Уграђени почетни упит

[inline-code-attrs-start title = 'Почетни упит шаблона резимера нити'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

Наредба "do not editorialize" је кључна. Без ње модел има тенденцију да користи формулацију "in my view" која лоше звучи под именом вашег налога.

### Окидачи

- **New comment posted** (`COMMENT_ADD`).
- **Trigger delay**: 30 minutes (1800 seconds). See [Одложени окидачи](#trigger-deferred-delay).

30-минутна задршка значи да агент покреће једну вожњу пола сата након што коментар стигне, према изгледу нити у том тренутку. То није "резимирај на сваки одговор" — ред одложених окидача сабира више догађаја новог коментара на истој нити, али их не де-дуплира преко одвојених прозора. Вероватно ћете желети да **додате прилагођено правило у свој упит** као што је "не објављуј нови резиме ако је агент већ резимирао ову нит у последњих 24 сата" и ослоните се на контекст плус агентове [алатке меморије](#tools-overview) да то спроведе.

### Дозвољене алатке

- [`write_comment`](#tools-overview) - објављује сам резиме.
- [`pin_comment`](#tools-overview) - закачи резиме да читаоци виде на врху нити.
- [`unpin_comment`](#tools-overview) - одкачи претходни резиме истог агента пре него што закачи нови.

Резимер не може модерирати или директно комуницирати са корисницима.

### Причвршћивање резимеа

Агент објави нови коментар помоћу `write_comment`, затим позове `pin_comment` са повратним ID-јем коментара. При наредним покретањима на истој нити, упит га упућује да прво позове `unpin_comment` на свом претходном резимеу — сама платформа не спроводи правило о једном закаченом коментару по нити, тако да остављање претходног резимеа закаченим резултује двема закаченим резимеима један поред другог. Означите опцију "Include parent comment and prior replies in the same thread" у [Опције контекста](#context-options) да би агент видео претходни закачени резиме.

### Препоручени додаци пре пуштања уживо

- **Означите "Include parent comment and prior replies in the same thread"** у [Опције контекста](#context-options). Резимер без контекста нити је бескористан.
- **Подесите правило за минималну величину нити.** "Fewer than 5 comments" је подразумевано у упиту, али у прометним заједницама 10–20 је прикладније. Уредите упит директно.
- **Ограничите на специфичне обрасце URL-ова** ако желите резиме само на дужим страницама, а не на најавама или страницама производа. Погледајте [Обим: Филтери URL-а и локалитета](#scope-url-locale).
- **Пазите на трошкове.** Резимирање троши највише токена јер чита целу нит при сваком покретању. Поставите строго [дневни буџет](#budgets-overview) пре него што укључите Enabled.

### Избегавање понављања резимеа

Агент има приступ `save_memory` и `search_memory` - можете проширити упит да га упутите да забележи белешке попут "summarized {thread urlId}" и провери их пре поновног објављивања. Меморија се дели међу свим агентима у вашем tenant-у.