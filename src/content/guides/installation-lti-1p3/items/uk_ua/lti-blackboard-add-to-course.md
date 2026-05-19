Once an administrator has registered FastComments as an LTI 1.3 Advantage tool and approved the institution policies, instructors add it to courses through the standard Blackboard placement points. The exact steps differ between Ultra Course View and Original Course View, so both are covered below.

#### Ultra Course View

Ultra Course View is the default in Blackboard Learn SaaS as of 2026.

1. Відкрийте курс і перейдіть на сторінку **Course Content**.
2. Наведіть курсор або торкніться місця в оглядовій структурі, куди ви хочете вставити потік коментарів, і натисніть фіолетову кнопку **+** (Add content).
3. Виберіть **Content Market**. Панель Content Market перелічує всі схвалені LTI-застосунки та розміщення Building Block для вашої установи.
4. Знайдіть плитку **FastComments** і натисніть її. Blackboard створює елемент вмісту в позиції, де ви відкрили меню **+**.
5. За замовчуванням елемент з’являється в огляді як запис «Visible to students» для викладачів, у яких персональна опція **Hide from students** вимкнена. Якщо у вас за замовчуванням встановлено **Hidden**, елемент створюється прихованим, і ви вмикаєте селектор видимості на рядку елемента, коли будете готові.
6. Щоб перейменувати елемент, натисніть заголовок в огляді та введіть нову мітку. Заголовок, який бачать студенти в огляді, незалежний від ідентифікатора потоку FastComments, тож перейменування безпечне в будь-який час.

Якщо ви не бачите **Content Market** як варіант, у вашій установі це розміщення приховано. Той самий вибір також доступний через **More tools** у тому самому меню **+** в групі **LTI Tools**.

#### Original Course View

Original Course View is still supported in Learn SaaS and remains the primary experience for self-hosted Learn 9.1 sites on the Q4 2024 CU release line.

1. Відкрийте курс і зайдіть у **Content Area** (наприклад, стандартну ділянку **Information** або **Content** у меню курсу).
2. Увімкніть **Edit Mode** за допомогою перемикача у верхньому правому куті сторінки.
3. Натисніть **Build Content** у панелі дій.
4. У підменю **Learning Tools** натисніть **FastComments**. Підменю Learning Tools заповнюється з розміщень інструментів LTI 1.3 після того, як адміністратор зареєструє інструмент. Якщо ви його не бачите, див. розділ про проблеми нижче.
5. На формі **Create FastComments** задайте:
   - **Name**: мітка, яку бачать студенти в області вмісту.
   - **Description**: необов’язковий текст, що відображається над вбудованим потоком.
   - **Permit Users to View this Content**: перемикач доступності Так/Ні.
   - **Track Number of Views**: увімкніть, якщо хочете статистику переглядів по елементу від Blackboard. FastComments веде власну аналітику незалежно.
   - **Date and Time Restrictions**: необов’язкові вікна **Display After** / **Display Until**.
6. Надішліть форму. Інструмент з’явиться як клікабельний елемент у області вмісту.

#### Embedding Inside an Item or Document

In both course views, instructors embed FastComments inline inside the body of an Item, Document, or any rich-text field through the Content Editor's LTI Advantage button.

Ultra Course View:

1. Створіть або відредагуйте **Document**.
2. Натисніть **Add content** всередині тіла документа в тому місці, де ви хочете, щоб з’явився потік.
3. У панелі інструментів редактора відкрийте меню **Insert content** і натисніть **Content Market** (точка входу LTI Advantage / Deep Linking).
4. Виберіть **FastComments**. FastComments повертає deep-link payload, і Blackboard вставляє вбудований блок у тіло документа в позицію курсора.
5. Збережіть документ. Студенти побачать потік відрендереним вбудовано під час прокручування сторінки.

Original Course View:

1. Редагуйте будь-який елемент з тілом у форматі rich-text.
2. У панелі інструментів Content Editor натисніть іконку плюс **Add Content** і оберіть **Content Market** (у старіших Q4 2024 CU ця опція маркована як **Add Content from External Tool**).
3. Виберіть **FastComments**. Редактор вставить заповнювач, що посилається на ресурс, пов’язаний через deep link.
4. Надішліть елемент.

Кожне deep-link вбудовування створює власний потік FastComments, тож елемент з двома вбудованими блоками FastComments матиме два незалежні потоки коментарів.

#### Visibility, Release Conditions, and Group Restrictions

FastComments content items behave like any other Blackboard content item for the access control rules layered on top of them.

- Ultra: натисніть селектор видимості на рядку (**Visible to students**, **Hidden from students**, **Conditional availability**). Conditional availability підтримує вікна дати/часу, правила продуктивності щодо елементів gradebook та правила членства щодо груп курсу.
- Original: відкрийте контекстне меню елемента і виберіть **Adaptive Release** або **Adaptive Release: Advanced**, щоб обмежити доступ до інструмента за датою, членством, оцінкою або статусом перегляду. Використовуйте **Set Group Availability** на елементі, щоб обмежити доступ певними групами курсу.

FastComments підпорядковується будь-яким правилам доступу Blackboard. Якщо Blackboard приховує елемент від студента, LTI-запуск для цього студента не відбувається і він не з’являється у вигляді модератора.

#### Gradebook Behavior

FastComments does not report grades back over LTI Advantage Assignment and Grade Services. No grade column is auto-created for FastComments content items.

Якщо ваш екземпляр Blackboard налаштований на автоматичне створення стовпця журналу оцінок для кожного нового елементу вмісту незалежно від метаданих оцінювання, порожній стовпець все одно з’явиться. Щоб приховати його:

- Ultra: відкрийте **Gradebook**, натисніть заголовок стовпця, виберіть **Edit** і вимкніть **Show to students** та **Include in calculations**. Або використайте **Delete**, якщо ваша установа дозволяє видалення стовпців для нефрагованих елементів.
- Original: відкрийте **Grade Center**, натисніть на шеврон стовпця, виберіть **Hide from Users (on/off)** і за потреби **Hide from Instructor View** у розділі **Column Organization**.

#### What Students See

When a student opens the FastComments item or scrolls to an embedded block:

1. Blackboard launches the LTI 1.3 message to FastComments. The student is signed in via SSO using their Blackboard identity (name, email, avatar, role) without seeing a login form.
2. The comment thread renders in the iframe. Threading, replies, mentions, and reactions are all available based on the comment widget settings configured in FastComments.
3. Their comments are attributed to their Blackboard account. If the student edits their name or photo in Blackboard later, the next launch updates the FastComments profile.

Role mapping from Blackboard to FastComments:

- **System Administrator** and **Course Builder** map to FastComments **admin**.
- **Instructor** and **Teaching Assistant** map to FastComments **moderator**.
- **Student**, **Guest**, and **Observer** map to FastComments **commenter**.

Moderators see moderation controls (pin, hide, ban, delete) inline on every comment in the thread.

#### Thread Scoping

FastComments scopes each thread by **(Blackboard host, course ID, resource link ID)**. Two FastComments items in the same course produce two threads. The same item copied across two course shells (for example, through course copy) produces two threads, because Blackboard issues a fresh resource link ID during the copy. To keep a shared thread across course copies, use Deep Linking with an explicit thread URN configured in FastComments before launching the copy.

#### Blackboard-Specific Gotchas

**FastComments tile missing from the Build Content menu (Original) or Content Market (Ultra).** Адміністратор схвалив інструмент, але залишив політику установи, яка блокує відповідне розміщення. Перейдіть у **Administrator Panel** > **Integrations** > **LTI Tool Providers**, відредагуйте запис FastComments і переконайтеся, що включені розміщення **Course Content Tool** (Original) та **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra). Збережіть і оновіть сторінку курсу.

**"Tool not configured for this context" or "Tool is not deployed" error on launch.** Область розгортання, зареєстрована під час динамічної реєстрації, не відповідає контексту установи, до якої належить курс. У записі провайдера інструментів Blackboard перевірте, чи **Deployment ID** збігається з тим, що FastComments показує на сторінці LTI 1.3 Configuration для цього тенанта. Якщо вони відрізняються, видаліть розміщення і повторіть динамічну реєстрацію з нового URL реєстрації (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">отримати його тут</a>).

**Iframe height looks fixed or content gets cut off.** Деякі екземпляри Blackboard постачаються зі строгим Content Security Policy, який блокує стандартне LTI iframe-resize postMessage. FastComments відправляє як Canvas-style `lti.frameResize` повідомлення, так і IMS spec-form `org.imsglobal.lti.frameResize` повідомлення для максимальної сумісності, але переважна CSP на рівні тенанта блокує прослуховувача у батьківському вікні. Попросіть вашого адміністратора підтвердити, що `*.fastcomments.com` додано до allowlist інструментів LTI і що жоден користувацький CSP-заголовок не видаляє події postMessage. Після цього зміна розміру працюватиме без додаткової конфігурації.

**Course copy duplicates threads.** Blackboard course copy генерує нові resource link ID для розміщень LTI, тож скопійовані курси починаються з порожніх потоків. Це очікувана поведінка. Якщо вам потрібно, щоб скопійований курс успадкував оригінальний потік, налаштуйте Deep Linking з явним thread URN перед копіюванням або зв’яжіться зі службою підтримки FastComments для масового переназначення ID потоків.

**Student sees a generic Blackboard error on launch.** Причина — відсутня або застаріла заява `email`. Переконайтеся, що політика установи для FastComments має увімкнені **Role**, **Name**, та **Email Address** у розділі **User Fields to Send**. Збережіть, потім знову відкрийте запуск у новій сесії браузера.