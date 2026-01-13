### Overview

FastComments Collab Chat розширює стандартний віджет коментарів FastComments, тому він успадковує всі параметри конфігурації від базового віджета, додаючи кілька специфічних для текстових анотацій.

### Required Configuration

#### tenantId

Ваш FastComments Tenant ID є обов'язковим. Ви можете знайти його у вашій [панелі керування FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Collab Chat Specific Options

#### urlId

За замовчуванням Collab Chat генерує унікальний ідентифікатор для кожної розмови на основі URL сторінки, шляху DOM до елемента та вибраного діапазону тексту. Ви можете перевизначити це за допомогою кастомного `urlId`.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Це корисно, коли структура ваших URL може змінюватися, але ви хочете зберегти ті самі розмови, або коли ви хочете ділитися анотаціями між кількома сторінками.

#### topBarTarget

Керує відображенням верхньої панелі, яка показує кількість користувачів та кількість дискусій. Встановіть `null`, щоб повністю вимкнути верхню панель, або передайте DOM-елемент, щоб відобразити її у конкретному місці.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Вимкнути верхню панель
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Відобразити верхню панель у користувацькому розташуванні
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Увімкніть стиль темного режиму, коли у вашої сторінки темний фон. Це виявлення виконується автоматично, але іноді може бути бажаним перевизначити його.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функція зворотнього виклику, яка викликається щоразу, коли змінюється кількість коментарів. Це корисно для оновлення елементів інтерфейсу, таких як бейджі або заголовки сторінок.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Inherited Configuration Options

Оскільки Collab Chat розширює стандартний віджет коментарів, ви можете використовувати будь-яку опцію конфігурації з базового віджета FastComments. Ось деякі часто використовувані опції:

#### locale

Встановіть мову інтерфейсу віджета. FastComments підтримує десятки мов.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Іспанська
});
[inline-code-end]

#### readonly

Зробіть усі розмови лише для читання. Користувачі можуть переглядати існуючі анотації, але не можуть створювати нові або відповідати.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Інтегруйтеся з вашою системою автентифікації за допомогою Single Sign-On.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Налаштування SSO
    }
});
[inline-code-end]

Див. документацію SSO для повних деталей щодо параметрів автентифікації.

#### maxReplyDepth

Керує тим, скільки рівнів вкладеності дозволено у відповідях. За замовчуванням Collab Chat встановлює це значення в 0, що означає, що всі коментарі плоскі (без вкладених відповідей). Ви можете змінити це, якщо хочете ниткові дискусії.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволити 3 рівні вкладеності
});
[inline-code-end]

### Internal Configuration

Ці опції автоматично встановлюються Collab Chat та не повинні перевизначатися:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Complete Example

Here's an example showing multiple configuration options together:

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(document.getElementById('article'), {
    tenantId: 'demo',
    urlId: 'my-article-v2',
    hasDarkBackground: false,
    locale: 'en',
    topBarTarget: document.getElementById('custom-header'),
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) My Article` : 'My Article';
    },
    sso: {
        // Ваша конфігурація SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.