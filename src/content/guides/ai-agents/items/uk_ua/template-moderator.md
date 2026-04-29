**Ідентифікатор шаблону:** `tos_enforcer`

The Moderator template is the recommended starting point if your goal is reducing manual moderation load. It reviews new and flagged comments and applies your community rules.

### Вбудована початкова підказка

[inline-code-attrs-start title = 'Початкова підказка шаблону модератора'; type='text' inline-code-attrs-end]
[inline-code-start]
Ви — агент із забезпечення дотримання умов обслуговування. Переглядайте кожен новий коментар відповідно до правил спільноти. Позначайте явний спам або порушення політики. Видавайте попередження за прикордонний контент при першому порушенні. Ескалюйте рішення про бан лише у випадку повторних або серйозних порушень. Якщо коментар явно відповідає правилам, затверджуйте його, щоб він став видимим (важливо для орендарів із передмодерацією). Утримуйтеся від політичних або суб'єктивних дебатів, зосередьтеся на правилах такими, якими вони записані.
[inline-code-end]

You will almost always want to **augment this prompt** with concrete examples of what your site does and does not allow. The platform's own escalation policy (warn before ban, search memory before banning) is already baked into the system prompt the agent receives, so you do not need to repeat it.

### Тригери

- **New comment posted** (`COMMENT_ADD`) - the agent looks at every new comment.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - the agent re-evaluates a comment that other users have flagged.

### Дозволені інструменти

- [`mark_comment_approved`](#tools-overview) - корисно для орендарів із передмодерацією, де агент публікує чисті коментарі й приховує решту.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Агент не може публікувати коментарі, голосувати, закріплювати, блокувати, присвоювати значки або надсилати електронні листи — підказка навмисно звужена.

### Рекомендовані доповнення перед запуском

- **Встановіть [Правила спільноти](#community-guidelines).** Декількох речень письмової політики достатньо; агент застосовує її при кожному запуску.
- **Обмежте застосування `ban_user` через [схвалення](#approval-workflow).** Це ввімкнено за замовчуванням у регіоні ЄС (див. [Відповідність ст. 17 DSA ЄС](#eu-dsa-compliance)) і рекомендовано у всіх регіонах.
- **Розгляньте також обмеження `mark_comment_spam` через схвалення**, якщо у вас низький обсяг, але високі ризики, пов'язані з контентом.
- **Обмежте `mark_comment_approved` через схвалення, якщо ви використовуєте передмодерацію.** Затвердження поганого коментаря ставить його перед читачами; обмежте цю можливість, поки агент не заслужить довіру під час тестового запуску.
- **Позначте "Включити фактор довіри автора коментаря, вік акаунта, історію банів та недавні коментарі"** в [Параметрах контексту](#context-options). Модель значно рідше буде попереджати, коли бачить, що хтось є давнім добросовісним користувачем.

### Рекомендований період тестового запуску

Запускайте цей шаблон у режимі [dry-run](#dry-run-mode) щонайменше тиждень на реальному трафіку перед тим, як переключити його в Увімкнено. Використайте [Тестові запуски (повтори)](#test-runs-replays) для попереднього перегляду також за останні 30 днів.

---