### Общ преглед

FastComments Collab Chat разширява стандартния коментарен уиджет на FastComments, така че наследява всички конфигурационни опции от базовия уиджет, като добавя няколко специфични за текстови анотации.

### Задължителна конфигурация

#### tenantId

Изисква се вашият FastComments Tenant ID. Можете да го намерите в [FastComments дашборда](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Специфични опции за Collab Chat

#### urlId

По подразбиране Collab Chat генерира уникален идентификатор за всяка беседа въз основа на URL адреса на страницата, DOM пътя до елемента и избрания текстов диапазон. Можете да го презапишете с персонализиран `urlId`.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Това е полезно, когато структурата на вашия URL може да се промени, но искате да запазите същите беседа, или когато искате да споделяте анотации между няколко страници.

#### topBarTarget

Контролира показването на горната лента, която показва броя на потребителите и беседите. Задайте `null`, за да деактивирате изцяло горната лента, или предоставете DOM елемент, за да я визуализирате на конкретно място.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Активирайте стилизиране в тъмен режим, когато страницата ви има тъмен фон. Това откриване е автоматично, но може да е желателно да го презапишете.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функция обратна връзка, която се задейства при всяка промяна на броя на коментарите. Това е полезно за обновяване на UI елементи като баджове или заглавия на страници.

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

### Наследени конфигурационни опции

Тъй като Collab Chat разширява стандартния коментарен уиджет, можете да използвате всяка конфигурационна опция от базовия уиджет на FastComments. Ето някои често използвани опции:

#### locale

Задайте езика за интерфейса на уиджета. FastComments поддържа десетки езици.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Направете всички беседа само за четене. Потребителите могат да преглеждат съществуващи анотации, но не могат да създават нови или да отговарят.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso и simpleSSO

Интегрирайте се със вашата система за автентикация чрез Single Sign-On.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Вижте SSO документацията за пълни подробности относно опциите за автентикация.

#### maxReplyDepth

Контролира на колко нива навътре могат да бъдат отговорите. По подразбиране Collab Chat задава това на 0, което означава, че всички коментари са плоски (без вложени отговори). Можете да промените това, ако искате разклонени беседа.

[inline-code-attrs-start title = "Пример за конфигурация"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Вътрешна конфигурация

Тези опции се задават автоматично от Collab Chat и не трябва да бъдат презаписвани:

`productId` се задава автоматично на `3` за Collab Chat. Разширението `floating-chat` се зарежда автоматично, за да осигури функционалността на чат прозореца. Уиджетът автоматично открива мобилни устройства (екрани под 768px ширина) и съответно адаптира интерфейса.

### Пълен пример

Ето пример, показващ няколко конфигурационни опции заедно:

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

За пълен списък на всички налични конфигурационни опции, наследени от базовия уиджет, вижте основната документация за конфигурация на FastComments.