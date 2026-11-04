### Обзор

FastComments Collab Chat расширяет стандартный виджет комментариев FastComments, поэтому он наследует все параметры конфигурации базового виджета, добавляя несколько специфичных для текстовых аннотаций.

### Обязательная конфигурация

#### tenantId

Требуется ваш Tenant ID FastComments. Его можно найти в вашей [панели управления FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Специфичные для Collab Chat параметры

#### urlId

По умолчанию Collab Chat генерирует уникальный идентификатор для каждого разговора на основе URL страницы, DOM-пути к элементу и выбранного диапазона текста. Вы можете переопределить это с помощью пользовательского `urlId`.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Это полезно, когда структура ваших URL может измениться, но вы хотите сохранить те же разговоры, или когда вы хотите делиться аннотациями между несколькими страницами.

#### topBarTarget

Управляет отображением верхней панели, которая показывает количество пользователей и обсуждений. Установите значение `null`, чтобы полностью отключить верхнюю панель, или укажите DOM-элемент, чтобы отобразить её в определённом месте.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Отключить верхнюю панель
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Отобразить верхнюю панель в пользовательском месте
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Включает стили тёмного режима, когда ваша страница имеет тёмный фон. Это определение происходит автоматически, но может потребоваться переопределить его вручную.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функция обратного вызова, которая срабатывает при каждом изменении количества комментариев. Это полезно для обновления элементов интерфейса, таких как значки или заголовки страниц.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Унаследованные параметры конфигурации

Поскольку Collab Chat расширяет стандартный виджет комментариев, вы можете использовать любой параметр конфигурации базового виджета FastComments. Вот некоторые часто используемые параметры:

#### locale

Устанавливает язык интерфейса виджета. FastComments поддерживает десятки языков.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Делает все разговоры доступными только для чтения. Пользователи могут просматривать существующие аннотации, но не могут создавать новые или отвечать.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso и simpleSSO

Интеграция с вашей системой аутентификации с использованием Single Sign-On.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Подробную информацию о параметрах аутентификации см. в документации по SSO.

#### maxReplyDepth

Управляет глубиной вложенности ответов. По умолчанию Collab Chat устанавливает значение 0, что означает, что все комментарии плоские (без вложенных ответов). Вы можете изменить это, если хотите использовать ветвящиеся разговоры.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Внутренняя конфигурация

Эти параметры устанавливаются Collab Chat автоматически и не должны переопределяться:

Параметр `productId` автоматически устанавливается в `3` для Collab Chat. Расширение `floating-chat` автоматически загружается для обеспечения функциональности окна чата. Виджет автоматически определяет мобильные устройства (экраны шириной менее 768px) и соответствующим образом адаптирует интерфейс.

### Полный пример

Вот пример, показывающий несколько параметров конфигурации вместе:

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Полный список всех доступных параметров конфигурации, унаследованных от базового виджета, см. в основной документации по конфигурации FastComments.