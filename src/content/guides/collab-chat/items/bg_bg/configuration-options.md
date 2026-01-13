### Преглед

FastComments Collab Chat разширява стандартния widget за коментари FastComments, така че наследява всички опции за конфигурация от базовия widget, като добавя няколко специфични за текстови анотации.

### Задължителна конфигурация

#### tenantId

Вашият FastComments Tenant ID е задължителен. Можете да го намерите в [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Опции, специфични за Collab Chat

#### urlId

По подразбиране Collab Chat генерира уникален идентификатор за всеки разговор въз основа на URL на страницата, DOM пътя до елемента и селектирания текстов диапазон. Можете да го презапишете с персонализиран `urlId`.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Това е полезно, когато структурата на вашите URL може да се промени, но искате да запазите същите разговори, или когато искате да споделяте анотации между множество страници.

#### topBarTarget

Контролира показването на горната лента, която показва брой потребители и брой дискусии. Задайте на `null`, за да деактивирате горната лента напълно, или подайте DOM елемент, в който да се рендира тя на конкретно място.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Изключване на горната лента
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Показване на горната лента в персонализирано място
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Активира стилизиране за тъмен режим, когато вашата страница има тъмен фон. Това откриване е автоматично, но може да искате да го презапишете.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функция обратен повиквач, която се извиква при всяка промяна на броя коментари. Това е полезно за обновяване на елементи от интерфейса като бейджове или заглавия на страницата.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Наследени опции за конфигурация

Тъй като Collab Chat разширява стандартния widget за коментари, можете да използвате всяка опция за конфигурация от базовия FastComments widget. Ето някои често използвани опции:

#### locale

Задайте езика за UI на widget-а. FastComments поддържа десетки езици.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Испански
});
[inline-code-end]

#### readonly

Направете всички разговори само за четене. Потребителите могат да преглеждат съществуващите анотации, но не могат да създават нови или да отговарят.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Интегрирайте със собствената си система за удостоверяване, използвайки Single Sign-On.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // Конфигурация на SSO
    }
});
[inline-code-end]

Вижте документацията за SSO за пълни детайли относно опциите за удостоверяване.

#### maxReplyDepth

Контролира колко нива дълбоко могат да бъдат отговорите. По подразбиране Collab Chat задава това на 0, което означава, че всички коментари са плоски (без вложени отговори). Можете да го промените, ако искате нишково водени разговори.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Позволява 3 нива на вложеност
});
[inline-code-end]

### Вътрешна конфигурация

Тези опции се задават автоматично от Collab Chat и не би трябвало да бъдат презаписвани:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Пълен пример

Ето един пример, показващ няколко конфигурационни опции заедно:

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // Вашата SSO конфигурация
    },
    maxReplyDepth: 1
});
[inline-code-end]

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.