A **окидач** је догађај који пробуди агента. Сваки агент може имати дефинисан један или више окidaча.

### The full list

| Trigger | When it fires |
|---|---|
| [Comment Added](#trigger-comment-add) | Објављен је нови коментар. |
| [Comment Edited](#trigger-comment-edit) | Коментар је измењен. Претходни текст је укључен у контекст агента. |
| [Comment Deleted](#trigger-comment-delete) | Коментар је обрисан. |
| [Comment Pinned](#trigger-comment-pin) | Коментар је закачен (од било кога, укључујући модератора или другог агента). |
| [Comment Unpinned](#trigger-comment-unpin) | Коментар је одкачен. |
| [Comment Locked](#trigger-comment-lock) | Коментар је закључан (није дозвољено даље одговарање). |
| [Comment Unlocked](#trigger-comment-unlock) | Коментар је откључан. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | Нето гласови коментара достижу конфигурисани праг. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | Број пријава за коментар достигне управо конфигурисани праг. |
| [User Posts First Comment](#trigger-new-user-first-comment) | Корисник објави свој први коментар на овом сајту. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | Коментар је аутоматски означен као спам од стране спам механизма. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | Модератор означи коментар као прегледан. |
| [Moderator Approves Comment](#trigger-moderator-approved) | Модератор одобри коментар. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | Модератор означи коментар као спам. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | Модератор додели значку кориснику. |

### Multiple triggers per agent

Агент може да се пријави за било коју комбинацију окidaча - на пример, [Moderator template](#template-moderator) се пријављује на оба `COMMENT_ADD` и `COMMENT_FLAG_THRESHOLD`. Сваки догађај активира агента по једном са контекстом тог догађаја.

### What stops an agent firing

Претплаћени догађај окidaча не активира агента ако важи било која од следећих ситуација:

- Агентов [status](#status-states) је **Disabled**.  
- Агентов [URL or locale scope](#scope-url-locale) се не поклапа са коментаром који покреће догађај.  
- Агентов [daily, monthly, or rate-limit budget](#budgets-overview) је исцрпљен - окidaч се евидентира као **одбачено** са разлогом. Види [Drop Reasons](#drop-reasons).  
- Конкурентност за овог агента је засићена (ограничено по агенту).  
- Тенант агента има неважећу наплату.  
- Покретачка радња је сама по себи извршена од стране бота или другог агента (спречавање петље).  
- Окидач се односи на коментар који је већ обрађен од стране овог агента унутар прозора за дедупликацију.

Када претплаћени окidaч успешно активира агента, агента [Run History](#run-history) приказује ред са статусом **Started** који прелази у **Success** или **Error** када извршавање заврши.

### Vote and flag thresholds

Два окidaча — **Comment Crosses Vote Threshold** и **Comment Crosses Flag Threshold** — захтевају нумерички праг у облику за уређивање. Окидач се активира у тренутку када број прелази конфигурисану вредност (конкретно, окidaч за праг пријава активира се када `flagCount === flagThreshold`, тако да избор 1 значи "активирати на прву пријаву", а избор 5 значи "активирати када стигне пета пријава").

### Deferred triggers

Било који окidaч може бити одложен тако да агент буде покренут касније, на пример након што гласови/пријаве/одговори имају времена да се стабилизују. Види [Deferred Triggers](#trigger-deferred-delay).

### Loop prevention

Да би се спречиле бесконачне петље, коментари које упише агент имају `botId`. Окидачи за нове коментаре игноришу коментаре са `botId`.

Нет ефекат: агенти могу да делују као одговор на *људске* акције у вашем тенанту, али акције које долазе од агената никада не активирају било који агентски окidaч. Ово важи за све типове окidaча.

### REPLAY: the internal trigger

Постоји и унутрашњи тип окidaча `REPLAY` који се користи у функцији [Test Runs (Replays)](#test-runs-replays). Не можете га изабрати на формулару за уређивање - постоји да би се реплеј покретања јасно означили у историји извршавања и искључили из приказа живог извршавања.