### Преглед

FastComments Collab Chat проширује стандардни FastComments видгет за коментаре, тако да наслеђује све опције конфигурације из основног видгета уз додавање неколико специфичних за текстуалне анотације.

### Обавезна конфигурација

#### tenantId

Потребан је ваш FastComments Tenant ID. Можете га пронаћи на вашој [FastComments контролној табли](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Опције специфичне за Collab Chat

#### urlId

По подразумеваном, Collab Chat генерише јединствени идентификатор за сваки разговор на основу URL странице, DOM путање до елемента и изабраног опсега текста. Ово можете превазићи прилагођеним `urlId`.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ово је корисно када се структура ваших URL-ова може промијенити али желите задржати исте разговоре, или када желите дијелити анотације преко више страница.

#### topBarTarget

Контролише приказ горње траке која показује број корисника и број дискусија. Поставите на `null` да у потпуности онемогућите горњу траку, или наведите DOM елемент да бисте је рендеровали на одређеном месту.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Омогућите стилове за тамни режим када ваша страница има тамну позадину. Ово се аутоматски детектује, али може бити пожељно да то ручно превазиђете.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функција повратног позива која се извршава кад год се број коментара промијени. Ово је корисно за ажурирање елемената корисничког интерфејса као што су ознаке или наслови страница.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
[inline-code-end]

### Насљедне опције конфигурације

Пошто Collab Chat проширује стандардни видгет за коментаре, можете користити било коју опцију конфигурације из основног FastComments видгета. Ево неких често кориштених опција:

#### locale

Подесите језик за кориснички интерфејс видгета. FastComments подржава десетине језика.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Шпански
});
[inline-code-end]

#### readonly

Учините све разговоре само за читање. Корисници могу прегледати постојеће анотације али не могу креирати нове или одговарати.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso and simpleSSO

Интегришите са вашим системом аутентификације користећи Single Sign-On.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO конфигурација
    }
});
[inline-code-end]

Погледајте документацију о SSO-у за детаљне информације о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа дубоко одговори могу ићи. По подразумеваном, Collab Chat поставља ово на `0`, што значи да су сви коментари равни (нема угњежђених одговора). Ово можете промијенити ако желите нитоване разговоре.

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволи 3 нивоа угнијежења
});
[inline-code-end]

### Интерна конфигурација

Ове опције се аутоматски подешавају од стране Collab Chat-а и не би требало да буду премашиване:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Потпуни примјер

Ево примјера који показује вишe опција конфигурације заједно:

[inline-code-attrs-start title = "Примјер конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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