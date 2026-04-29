**Идентификатор шаблона:** `welcome_greeter`

Welcome Greeter топло поздравља кориснике који коментаришу први пут. То је најмање ризичан шаблон (нема деструктивних алата) и добар први агент за пуштање уживо.

### Уграђени почетни упит

[inline-code-attrs-start title = 'Почетни упит шаблона Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Окидачи

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

Овај догађај се активира тачно једном по кориснику, тако да агент не може ући у петљу. Погледајте [Окидач: Први коментар новог корисника](#trigger-new-user-first-comment).

### Дозвољени алати

- [`write_comment`](#tools-overview)

То је једини алат — агент буквално не може модерирати, гласати, забранити или слати директне поруке (DM).

### Препоручени додаци пре пуштања уживо

- **Поставите приказано име** на нешто привлачно - "Community Bot", маскота вашег сајта, или име вашег бренда. Приказано име је оно што читаоци виде повезано са поздравном поруком.
- **Означите "Include page title, subtitle, description, and meta tags"** у [Context Options](#context-options). Одговори поздрављача постају приметно бољи када може да се позове на то о чему страница заправо говори.
- **Размотрите ограничења по локалитету** ако радите на више језика. Поздравна порука на погрешном језику је упечатљивија од пропуштеног одговора. Погледајте [Опсег: Филтри URL и локалитета](#scope-url-locale).

### Зашто одобрења нису потребна

Агент само пише нове коментаре и само на једнократни окидач. У најгорем случају: непријатан поздрав. Нема деструктивне радње коју треба контролисати. Већина оператора га покреће без икаквих одобрења након што пробни рад изгледа у реду.

---