**ID на шаблона:** `tos_enforcer`

Шаблонът Модератор е препоръчителната начална точка, ако целта ви е да намалите натоварването от ръчна модерация. Той преглежда нови и маркирани коментари и прилага правилата на вашата общност.

### Вградена начална подсказка

[inline-code-attrs-start title = 'Начална подсказка за шаблона Модератор'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Почти винаги ще искате да **допълните тази подсказка** с конкретни примери за това какво вашият сайт позволява и какво не. Политиката за ескалация на платформата (предупреди преди бан, търси в паметта преди забрана) вече е вградена в системната подсказка, която агентът получава, така че не е нужно да я повтаряте.

### Тригери

- **Нов коментар публикуван** (`COMMENT_ADD`) - агентът преглежда всеки нов коментар.
- **Коментарът премина прага за флагове** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - агентът преоценява коментар, който други потребители са маркирали.

### Позволени инструменти

- [`mark_comment_approved`](#tools-overview) - полезен за наематели с предварителна модерация, където агентът пуска одобрените коментари и скрива останалите.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Не може да публикува коментари, да гласува, да закрепя, да заключва, да присъжда значки или да изпраща имейли - подсказката е умишлено ограничена.

### Препоръчителни допълнения преди пускане на живо

- **Задайте [Насоките на общността](#community-guidelines).** Няколко изречения с писмена политика са достатъчни; агентът ги прилага при всяко изпълнение.
- **Ограничете `ban_user` чрез [одобрение](#approval-workflow).** Това е включено по подразбиране в региона на ЕС (вижте [EU DSA Article 17 Compliance](#eu-dsa-compliance)) и е препоръчително навсякъде.
- **Помислете също да ограничите `mark_comment_spam` чрез одобрение**, ако имате малък обем, но високорисково съдържание.
- **Ограничете `mark_comment_approved` чрез одобрение, ако използвате предварителна модерация.** Одобряването на лош коментар го поставя пред четящите; ограничете го, докато агентът не е заслужил доверие чрез dry-run.
- **Поставете отметка на "Include commenter's trust factor, account age, ban history, and recent comments"** в [Опции на контекста](#context-options). Моделът ще предупреждава значително по-малко агресивно, когато може да види, че някой е дългогодишен потребител с добронамерено поведение.

### Препоръчителен период за dry-run

Изпълнете този шаблон в [dry-run](#dry-run-mode) за поне една седмица срещу вашия реален трафик, преди да превключите на Enabled. Използвайте [Test Runs (Replays)](#test-runs-replays), за да направите предварителен преглед и спрямо последните 30 дни.

---