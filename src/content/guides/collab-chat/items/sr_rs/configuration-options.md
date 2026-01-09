### Преглед

FastComments Collab Chat проширује стандардни FastComments коментарски видгет, тако да наслеђује све опције конфигурације из основног видгета док додаје неколико специфичних за текстуалне анотације.

### Обавезна конфигурација

#### tenantId

Ваш FastComments Tenant ID је обавезан. Можете га пронаћи у вашој [FastComments контролна табла](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Опције специфичне за Collab Chat

#### urlId

По подразумеваној вредности, Collab Chat генерише јединствени идентификатор за сваки разговор на основу URL странице, DOM пута до елемента и изабраног опсега текста. Можете то преопределити коришћењем прилагођеног `urlId`.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ово је корисно када ваша структура URL-ова може да се промени али желите да задржите исте разговоре, или када желите да делите анотације преко више страница.

#### topBarTarget

Контролише приказ горње траке која показује број корисника и број дискусија. Поставите на `null` да у потпуности онемогућите горњу траку, или обезбедите DOM елемент да бисте је приказали на одређеној локацији.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Онемогући горњу траку
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Прикажи горњу траку на прилагођеној локацији
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Омогућите стиловање у тамном режиму када ваша страница има тамну позадину. Ова детекција је аутоматска, али може бити пожељно да је преопределите.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функција повратног позива која се позива кад год се број коментара промени. Ово је корисно за ажурирање UI елемената као што су значке или наслови страница.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Наслеђене опције конфигурације

Пошто Collab Chat проширује стандардни коментарски видгет, можете користити било коју опцију конфигурације из основног FastComments видгета. Ево неких често коришћених опција:

#### locale

Поставите језик за UI видгета. FastComments подржава десетине језика.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Шпански
});
[inline-code-end]

#### readonly

Учини све разговоре само за читање. Корисници могу да прегледају постојеће анотације али не могу да креирају нове или да одговарају.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Интегришите се са вашим системом аутентификације користећи Single Sign-On.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO конфигурација
    }
});
[inline-code-end]

Погледајте SSO документацију за пуне детаље о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа дубоко одговора је дозвољено. По подразумеваној вредности, Collab Chat поставља ово на 0, што значи да су сви коментари равни (без уметнутих одговора). Можете то изменити ако желите нитоване разговоре.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволи 3 нивоа угнежђивања
});
[inline-code-end]

### Интерна конфигурација

Ове опције аутоматски поставља Collab Chat и не би требало да их преопределите:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Комплетан пример

Ево примера који приказује више конфигурационих опција заједно:

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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
        // Ваша SSO конфигурација
    },
    maxReplyDepth: 1
});
[inline-code-end]

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.