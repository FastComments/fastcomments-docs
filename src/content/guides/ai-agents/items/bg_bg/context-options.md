The **Context** секцията в формата за редакция контролира колко информация получава агентът при всяко изпълнение. Повече контекст води до по-добри решения, но увеличава разхода на токени за изпълнение, затова искате само това, от което агентът наистина се нуждае.

### What's always included

Дори при всички отмаркирани квадратчета, контекстното съобщение към агента включва:

- The **trigger event type** (e.g. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- The **page URL and URL ID** (when known).
- The **comment** that triggered the run, if there is one - ID, author user ID, author display name, comment text, vote counts, flag count, spam/approved/reviewed flags, parent ID. The author's email is **never** sent to the LLM provider (минимизиране на лични данни (PII)).
- The **previous comment text** for `COMMENT_EDIT` triggers (so the agent can compare before/after).
- The **vote direction** for `COMMENT_VOTE_THRESHOLD` triggers.
- The **triggering user ID** and **badge ID** (for moderator badge triggers).

All untrusted text - comment bodies, author names, page titles, the guidelines doc itself - is **fenced** in the context message with markers like `<<<COMMENT_TEXT>>> ... <<<END>>>`. Платформеният системен prompt указва на модела да не следва инструкции, намиращи се вътре в тези ограждения. Това е защитата на платформата срещу prompt-injection; не е нужно да я повтаряте във вашия prompt.

### The three checkboxes

#### Include parent comment and prior replies in the same thread

Добавя:
- The **parent comment** - ID, author, text.
- **Sibling replies** - the prior replies to the same parent in the same thread.

Полезно за: всеки агент, който отговаря на коментар в контекст (приветстващи агенти, обобщаващи нишки, модератори, които четат отговори в разговори).

Разход: малък до среден. Ограничен от броя на отговорите на един и същи родител в дадена нишка.

#### Include commenter's trust factor, account age, ban history, and recent comments

Добавя блока **AUTHOR_HISTORY**:

- **Account age in days** since signup.
- **Trust factor (0-100)** - the FastComments score that summarizes how trusted the user is on this site. See the [Spam Detection](/guide-moderation.html#spam-detection) page in the moderation guide.
- **Prior ban count.**
- **Total comments on this site.**
- **Duplicate-content count** - if the user has posted identical text recently (anti-spam signal).
- **Same-IP cross-account signal** - count of comments from the same IP under other accounts (alt-account signal). The IP hash itself is never sent to the LLM.
- **Recent comments** - up to 5 of the user's most recent comments, each truncated to 300 characters, fenced as untrusted text.

Полезно за: всеки модераторски агент. Без тази информация моделът банва нови акаунти и дългогодишни добросъвестни потребители с една и съща позиция.

Разход: среден. Последните коментари добавят най-много токени.

#### Include page title, subtitle, description, and meta tags

Добавя блока **PAGE_CONTEXT** - title, subtitle, description, and any meta tags FastComments has captured for the page.

Полезно за: приветстващи агенти и обобщители на нишки, където знанието за темата на страницата значително подобрява качеството на изхода.

Разход: малък.

### Community guidelines

Четвъртото поле, **Community guidelines**, е текстово поле за политика, което се включва в контекстното съобщение с роля user при всяко изпълнение, оградено като ненадежден текст по същия начин, по който са оградени телата на коментари и другото съдържание, предоставено от потребителя. Агентът го чете като политически текст, но платформата не го третира като системна инструкция. Вижте [Правила на общността](#community-guidelines) за какво да включите в него.

### Adding context selectively

Тези квадратчета важат за всеки агент поотделно, а не глобално. Един често срещан модел:

- Welcome greeter: page context **on**, thread context **off**, user history **off**.
- Moderator: thread context **off**, user history **on**, page context **off**.
- Thread summarizer: thread context **on**, page context **on**, user history **off**.

Стремете се към минималния контекст, от който агентът се нуждае, за да е правилен при повикванията, които всъщност прави - допълнителният контекст струва токени при всяко изпълнение, дори когато агентът не го използва.