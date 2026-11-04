### Огляд

FastComments Collab Chat розширює стандартний віджет коментування FastComments, тому він успадковує всі параметри конфігурації базового віджета, додаючи кілька специфічних для текстових анотацій.

### Необхідна конфігурація

#### tenantId

Потрібен ваш FastComments Tenant ID. Ви можете знайти його у своєму [дашборді FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Специфічні параметри Collab Chat

#### urlId

За замовчуванням Collab Chat генерує унікальний ідентифікатор для кожної розмови на основі URL сторінки, DOM-шляху до елемента та виділеного фрагмента тексту. Ви можете перевизначити це за допомогою власного `urlId`.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Це корисно, коли структура ваших URL може змінитися, але ви хочете зберегти ті самі розмови, або коли хочете ділитися анотаціями між кількома сторінками.

#### topBarTarget

Керує відображенням верхньої панелі, яка показує кількість користувачів і обговорень. Встановіть значення `null`, щоб повністю вимкнути верхню панель, або надайте DOM-елемент, щоб відобразити її в певному місці.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Disable top bar
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Render top bar in custom location
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Увімкніть стилізацію темного режиму, якщо ваша сторінка має темний фон. Це визначення відбувається автоматично, але може бути бажаним перевизначити його.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функція зворотного виклику, яка спрацьовує щоразу, коли змінюється кількість коментарів. Це корисно для оновлення елементів інтерфейсу, таких як значки або заголовки сторінок.

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

### Успадковані параметри конфігурації

Оскільки Collab Chat розширює стандартний віджет коментування, ви можете використовувати будь-який параметр конфігурації базового віджета FastComments. Ось деякі часто використовувані параметри:

#### locale

Встановіть мову інтерфейсу віджета. FastComments підтримує десятки мов.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Зробіть усі розмови доступними лише для читання. Користувачі можуть переглядати наявні анотації, але не можуть створювати нові чи відповідати.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso та simpleSSO

Інтегруйтеся зі своєю системою автентифікації за допомогою Single Sign-On.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Повну інформацію про параметри автентифікації дивіться в документації SSO.

#### maxReplyDepth

Керуйте тим, наскільки рівнів углиб можуть йти відповіді. За замовчуванням Collab Chat встановлює це значення на 0, що означає, що всі коментарі є плоскими (без вкладених відповідей). Ви можете змінити це, якщо хочете мати розмови з гілками.

[inline-code-attrs-start title = "Приклад конфігурації"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Внутрішня конфігурація

Ці параметри встановлюються автоматично Collab Chat і їх не слід перевизначати:

`productId` автоматично встановлюється на `3` для Collab Chat. Розширення `floating-chat` автоматично завантажується для забезпечення функціональності вікна чату. Віджет автоматично визначає мобільні пристрої (екрани шириною менше 768px) і відповідно адаптує інтерфейс.

### Повний приклад

Ось приклад, що показує кілька параметрів конфігурації разом:

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

Повний список усіх доступних параметрів конфігурації, успадкованих від базового віджета, дивіться в основній документації конфігурації FastComments.