### Обзор

FastComments Collab Chat расширяет стандартный виджет комментариев FastComments, поэтому он наследует все параметры конфигурации базового виджета и добавляет несколько, специфичных для текстовых аннотаций.

### Обязательная конфигурация

#### tenantId

Требуется ваш Tenant ID FastComments. Вы можете найти его на своей [панели управления FastComments](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Специфические опции Collab Chat

#### urlId

По умолчанию Collab Chat генерирует уникальный идентификатор для каждого разговора на основе URL страницы, пути DOM к элементу и выбранного диапазона текста. Вы можете переопределить это, задав собственный `urlId`.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Это полезно, когда структура ваших URL может меняться, но вы хотите сохранить те же разговоры, или когда вы хотите поделиться аннотациями между несколькими страницами.

#### topBarTarget

Управляет отображением верхней панели, которая показывает количество пользователей и количество обсуждений. Установите в `null`, чтобы полностью отключить верхнюю панель, или укажите DOM-элемент, в котором её отрисовать в конкретном месте.

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

Включает стили для тёмной темы, когда у вашей страницы тёмный фон. Это определение происходит автоматически, но иногда может потребоваться переопределить его.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функция обратного вызова, которая вызывается всякий раз, когда изменяется количество комментариев. Это полезно для обновления элементов интерфейса, таких как бейджи или заголовки страниц.

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

### Наследуемые опции конфигурации

Поскольку Collab Chat расширяет стандартный виджет комментариев, вы можете использовать любые параметры конфигурации из базового виджета FastComments. Вот некоторые часто используемые опции:

#### locale

Установите язык интерфейса виджета. FastComments поддерживает десятки языков.

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

Интеграция с вашей системой аутентификации с использованием единого входа (Single Sign-On).

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Конфигурация SSO
    }
});
[inline-code-end]

См. документацию по SSO для полного описания опций аутентификации.

#### maxReplyDepth

Контролирует, насколько глубоко могут вкладываться ответы. По умолчанию Collab Chat устанавливает это значение в 0, что означает плоскую структуру комментариев (без вложенных ответов). Вы можете изменить это, если хотите потоковые (threaded) обсуждения.

[inline-code-attrs-start title = "Пример конфигурации"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Разрешить 3 уровня вложенности
});
[inline-code-end]

### Внутренняя конфигурация

Эти параметры автоматически устанавливаются Collab Chat и не должны переопределяться:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Полный пример

Вот пример, демонстрирующий несколько опций конфигурации одновременно:

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

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.