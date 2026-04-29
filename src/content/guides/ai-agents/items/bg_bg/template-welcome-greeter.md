**ID на шаблона:** `welcome_greeter`

The Welcome Greeter replies warmly to first-time commenters. It is the lowest-risk template (no destructive tools) and a good first agent to ship live.

### Вградена начална подсказка

[inline-code-attrs-start title = 'Начална подсказка за шаблона Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Тригери

- **Нов потребител публикува първия си коментар на този сайт** (`NEW_USER_FIRST_COMMENT`).

This event fires exactly once per user, so the agent cannot loop. See [Тригер: Първи коментар на нов потребител](#trigger-new-user-first-comment).

### Разрешени инструменти

- [`write_comment`](#tools-overview)

Това е единственият инструмент — агентът буквално не може да модерира, да гласува, да блокира или да изпраща директни съобщения.

### Препоръчителни допълнения преди пускане на живо

- **Задайте Показвано име** на нещо привлекателно - "Community Bot", вашият талисман на сайта или името на вашата марка. Показваното име е това, което читателите виждат прикачено към приветствения отговор.
- **Поставете отметка на "Include page title, subtitle, description, and meta tags"** в [Context Options](#context-options). Отговорите на приветстващия се подобряват значително, когато той може да се позове на това за какво всъщност е страницата.
- **Помислете за ограничения по локал** ако оперирате на няколко езика. Приветствен отговор на грешния език е по-неприятен от пропуснатия отговор. Вижте [Обхват: Филтри за URL и локал](#scope-url-locale).

### Защо не са нужни одобрения

Агентът само пише нови коментари и само при еднократен тригер. В най-лошия случай: неловко поздравление. Няма разрушително действие, което да трябва да се контролира. Повечето оператори пускат този без одобрения, след като тестовото изпълнение изглежда добре.

---