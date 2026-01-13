### Обзор

FastComments Collab Chat расширяет стандартный виджет комментариев FastComments, поэтому он наследует все параметры конфигурации базового виджета и добавляет несколько специфичных для текстовых аннотаций.

### Требуемая конфигурация

#### tenantId

Требуется ваш FastComments Tenant ID. Вы можете найти его в вашей [панели управления FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Параметры, специфичные для Collab Chat

#### urlId

По умолчанию Collab Chat генерирует уникальный идентификатор для каждого разговора на основе URL страницы, пути DOM к элементу и выбранного диапазона текста. Вы можете переопределить это, указав собственный `urlId`.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Это полезно, когда структура ваших URL может изменяться, но вы хотите сохранить те же обсуждения, или когда вы хотите делиться аннотациями между несколькими страницами.

#### topBarTarget

Управляет отображением верхней панели, которая показывает количество пользователей и количество обсуждений. Установите `null`, чтобы полностью отключить верхнюю панель, или передайте DOM-элемент, чтобы отрисовать её в конкретном месте.

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

Включает оформление в тёмном режиме, когда на странице тёмный фон. Это определяется автоматически, но вы можете переопределить это при необходимости.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функция обратного вызова, которая вызывается всякий раз, когда изменяется количество комментариев. Это полезно для обновления элементов интерфейса, таких как бейджи или заголовки страницы.

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

Поскольку Collab Chat расширяет стандартный виджет комментариев, вы можете использовать любые параметры конфигурации из базового виджета FastComments. Вот некоторые часто используемые параметры:

#### locale

Установите язык пользовательского интерфейса виджета. FastComments поддерживает десятки языков.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Испанский
});
[inline-code-end]

#### readonly

Сделать все обсуждения доступными только для чтения. Пользователи могут просматривать существующие аннотации, но не могут создавать новые или отвечать.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Интегрируйте с вашей системой аутентификации, используя Single Sign-On.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Конфигурация SSO
    }
});
[inline-code-end]

Смотрите документацию по SSO для полного описания параметров аутентификации.

#### maxReplyDepth

Контролируйте, насколько глубоко могут быть вложены ответы. По умолчанию Collab Chat устанавливает это значение в 0, что означает, что все комментарии плоские (без вложенных ответов). Вы можете изменить это, если хотите получить древовидные обсуждения.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Разрешить 3 уровня вложенности
});
[inline-code-end]

### Внутренняя конфигурация

Эти параметры автоматически устанавливаются Collab Chat и не должны переопределяться:

Параметр `productId` автоматически устанавливается в `3` для Collab Chat. Расширение `floating-chat` автоматически загружается для обеспечения функциональности окна чата. Виджет автоматически определяет мобильные устройства (экраны шириной менее 768px) и соответственно корректирует интерфейс.

### Полный пример

Ниже приведён пример, показывающий несколько параметров конфигурации вместе:

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
        // Ваша конфигурация SSO
    },
    maxReplyDepth: 1
});
[inline-code-end]

Для полного списка всех доступных параметров конфигурации, унаследованных от базового виджета, смотрите основную документацию по конфигурации FastComments.