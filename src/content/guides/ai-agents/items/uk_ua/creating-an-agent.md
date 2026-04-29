From the [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) ви можете створити агента двома способами:

- **From a template.** Натисніть **Browse templates** і виберіть один із чотирьох вбудованих стартових агентів. Форма відкривається із заповненими полями, а статус агента — **Dry Run**. Див. [Starter Templates](#starter-templates).
- **From scratch.** Натисніть **Create new agent**. Форма відкривається порожньою.

У будь-якому випадку одна й та сама форма використовується для збереження та подальшого редагування. Ця сторінка описує форму зверху донизу.

### Basics

- **Internal name.** Короткий ідентифікатор, що використовується тільки в адміністративних панелях (історія запусків, аналітика, журнали аудиту). Підходить нижній регістр з підкресленнями: `moderator`, `welcome_greeter`. Якщо внутрішня назва шаблону вже зайнята, форма автоматично додає суфікс (`tos_enforcer_2`, тощо).
- **Display name.** Відображається публічно щоразу, коли агент публікує коментар. Це те, що бачать ваші читачі.
- **Status.** Disabled, Dry Run, or Enabled. Нові агенти за замовчуванням завжди в Dry Run. Див. [Status States](#status-states).

### Model

Виберіть LLM. Див. [Choosing a Model](#choosing-a-model).

### Budget

Необов’язкові денні та місячні ліміти у валюті вашого акаунта, плюс чекліст **Alert thresholds** (за замовчуванням 80% та 100%). Див. [Budgets Overview](#budgets-overview) та [Budget Alerts](#budget-alerts).

### Personality

**Initial prompt** — це системний prompt, який визначає тон, роль та правила прийняття рішень. Простий текст, без шаблонної синтаксису. Див. [Personality and the Initial Prompt](#personality-prompt).

### Context

Поле Context містить три прапорці, текстове поле з інструкціями та поля для обмеження області застосування:

- Include parent comment and prior replies in the same thread.
- Include the commenter's trust factor, account age, ban history, and recent comments.
- Include page title, subtitle, description, and meta tags.
- Необов’язковий текстовий блок **Community guidelines**, який додається перед кожним prompt.
- **Restrict to specific pages** — allowlist шаблонів URL (по одному на рядок). Порожнє поле означає, що правило застосовується до всього орендаря (tenant-wide).
- **Restrict to specific locales** — allowlist локалей через двобічний селектор. Порожнє поле означає усі локалі.

Більше контексту дає кращі рішення, але підвищує вартість токенів за запуск. Див. [Context Options](#context-options), [Community Guidelines](#community-guidelines) та [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Виберіть принаймні одну подію зі списку. Для тригерів vote-threshold та flag-threshold потрібно також задати поріг. Необов’язкове поле **Delay before running** відкладає виконання після спрацьовування тригера (корисно для порогів флагів, де голоси ще підраховуються). Див. [Trigger Events Overview](#triggers-overview) та [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Позначте **Allow any tool calls**, щоб відкрити повну палітру інструментів. Інакше відмітьте конкретні інструменти, які агенту дозволено використовувати — заборонені інструменти видаляються з палітри моделі і відхиляються під час диспетчеризації. Підсекція **Ban options** захищає деструктивні варіанти бану (delete-all-comments, ban-by-IP) за допомогою явної згоди. Див. [Allowed Tool Calls Overview](#tools-overview) та [Tool: ban_user](#tool-ban-user).

### Approvals

Позначте дії, які повинні бути затверджені людиною перед тим, як агент їх виконає. Затвердження застосовуються лише до інструментів, які агенту дозволено викликати; заборонені інструменти відхиляються відразу. В регіоні ЄС **ban_user** заблокований відповідно до статті 17 Digital Services Act. Див. [Approval Workflow](#approval-workflow) та [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

Якщо затвердження увімкнені, оберіть, кому надсилаються листи:

- **All admins and moderators** - власники акаунта, супер-адміни та адміністратори модерації коментарів.
- **Specific users** - вибираються вручну через двобічний селектор.

Індивідуальна частота доставки для кожного рецензента (негайно, щогодинний дайджест, щоденний дайджест) налаштовується в їхньому профілі. Див. [Approval Notifications](#approval-notifications).

### Stats

Тільки для читання. Загальна кількість запусків, позначка часу останнього запуску та ID найостаннішого коментаря, який написав агент (якщо такий є).

### Save

Натисніть **Save agent**. Сторінка перенаправить назад до списку агентів. Нові агенти відразу стають придатними для отримання тригерів у Dry Run.

### Editing later

Кожний рядок на сторінці списку агентів містить дії для конкретного агента: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, та **Delete**. Редагування агента не впливає на вже записані запуски — історія зберігається. Реплей-знімки також заморожують конфігурацію агента на момент початку реплею, тому результати збереженого реплею залишаються відтворюваними навіть після редагування prompt.