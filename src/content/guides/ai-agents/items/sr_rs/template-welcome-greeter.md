**Template ID:** `welcome_greeter`

Welcome Greeter топло одговара први пут коментаторима. То је шаблон најниже ризика (без деструктивних алата) и добар први агент за пуштање у живо.

### Тригери

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

Овај догађај се покреће тачно једном по кориснику, тако да агент не може да се понавља. Погледајте [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Дозвољени алати

- [`write_comment`](#tools-overview)

То је једини алат – агент заиста не може да модерише, гласа, банује или шаље директне поруке.

### Препоручена додавања пре пуштања у живо

- **Set the Display name** to something inviting - "Community Bot", your site mascot, or your brand name. The display name is what readers see attached to the welcome reply.  
  **Подесите име за приказ** на нешто привлачно – „Community Bot“, маскоту вашег сајта или име вашег бренда. Име за приказ је оно што читаоци виде уз поздравни одговор.

- **Tick "Include page title, subtitle, description, and meta tags"** in [Context Options](#context-options). The greeter's replies become noticeably better when it can reference what the page is actually about.  
  **Означите „Укључи наслов странице, поднаслов, опис и мета ознаке“** у [Context Options](#context-options). Одговори греетера постају приметно бољи када може да се позове на то о чему је страница заиста.

- **Consider locale restrictions** if you operate in multiple languages. A welcome reply in the wrong language is more jarring than a missed reply. See [Scope: URL and Locale Filters](#scope-url-locale).  
  **Размотрите ограничења локала** ако радите на више језика. Поздравни одговор на погрешном језику је јарши од пропуштеног одговора. Погледајте [Scope: URL and Locale Filters](#scope-url-locale).

### Зашто није потребно одобрење

Агент само пише нове коментаре и то само на једнократном тригеру. Најгоре: незгодајни поздрав. Не постоји деструктивна радња која би се требало контролисати. Већина оператора користи овај без икаквих одобрења након што тестирање без утицаја изгледа чисто.