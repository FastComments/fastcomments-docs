FastComments постачає п'ять стартових шаблонів, тож вам не потрібно писати робочого агента з нуля. Вони доступні на [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents), натиснувши **Browse templates**.

When you pick a template:

1. The agent is created with **Status: Dry Run** and an internal name based on the template (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). If that name is taken on your tenant, a numeric suffix is added.
2. Ви потрапляєте безпосередньо на форму редагування з усім заповненим — prompt, triggers, allowed actions та будь-які пороги. Банер у верхній частині читає "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Поки що нічого не ввімкнено. Агент не діятиме, доки ви не збережете його і або не залишите dry-run ввімкненим (щоб спостерігати), або не перемкнете на Enabled.

### The five templates

- **[Moderator](#template-moderator)** - переглядає нові та відмічені коментарі, попереджає користувачів, які порушили вперше, підвищує міру до бана тільки після попередження. Спрацьовує на нові коментарі та при перевищенні порогу флагів (порог за замовчуванням: 3). Дозволені інструменти: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - тепло відповідає першокористувачам коротким персоналізованим привітанням. Спрацьовує на new-user-first-comment. Дозволений інструмент: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - закріплює змістовні коментарі верхнього рівня після того, як вони перетинають поріг голосів (порог за замовчуванням: 10), спочатку відкріплюючи раніше закріплений коментар. Спрацьовує при перетині порогу голосів. Дозволені інструменти: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - публікує нейтральний однопараграфний підсумок у довгих потоках після затримки, а потім закріплює його. Спрацьовує на нові коментарі з відстрочкою 30 хвилин, щоб дискусія вщухла перед підсумуванням. Дозволені інструменти: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - відстежує редагування коментарів на предмет перезаписів у середині нитки, які спотворюють відповіді, відновлює оригінальний текст і надсилає особисте повідомлення автору. Спрацьовує на редагування коментарів. Дозволені інструменти: `edit_comment`, `warn_user`, `send_dm`.

### Customizing a template

Шаблони — це точки відліку, а не контракт. Очікується, що ви:

- Підлаштуєте **Initial prompt** під голос вашої спільноти.
- Додасте або видалите **Triggers**, щоб підігнати частоту запусків агента.
- Додасте **Approvals** для будь-якої чутливої дії — ми настійно рекомендуємо захищати `ban_user` механізмом підтвердження для шаблонів у стилі модерації.
- Додасте **Community guidelines**, щоб агент послідовно застосовував вашу письмову політику. Див. [Community Guidelines](#community-guidelines).
- Встановите на агента відповідні **Budgets** залежно від очікуваної кількості тригерів.

Шаблон — це лише засіб, який заповнює розумні значення за замовчуванням; після збереження агент належить вам.