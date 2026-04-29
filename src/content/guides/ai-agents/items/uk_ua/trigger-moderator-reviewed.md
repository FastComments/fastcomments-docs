Спрацьовує, коли модератор позначає коментар як reviewed.

### Контекст, який отримує агент

- Коментар.
- **triggering user ID** — модератор, який позначив коментар як reviewed.
- За потреби: історія треду / користувача / сторінки відповідно до налаштувань.

### Хто викликає цю подію

Дія людини-модератора на сторінці модерації, у віджеті коментаря або через API.

### Типові випадки використання

- **Пересилання аудиту** через [Webhooks](#webhooks-overview).
- **Memory writes** — зареєструвати примітку в пам'яті, що цей коментар було переглянуто людиною, щоб інші агенти не обробляли його двічі.

### Зауваження

- "Reviewed" is one of the moderation queue states tracked separately from "approved" and "spam". A comment can be approved-and-reviewed, approved-but-not-reviewed, etc. See [How Approvals Work](/guide-moderation.html#moderation-approvals) in the moderation guide.
- Цей тригер має високу частоту на tenants з великою кількістю модераторів. Підписуйтеся вибірково і плануйте бюджет відповідно.