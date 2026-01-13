### Преглед

FastComments Collab Chat проширује стандардни FastComments видгет за коментаре, па наслеђује све опције конфигурације основног видгета док додаје неколико специфичних за текстуалне анотације.

### Потребна конфигурација

#### tenantId

Потребан је ваш FastComments Tenant ID. Можете га наћи у вашем [FastComments контролној табли](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Опције специфичне за Collab Chat

#### urlId

По подразумевању, Collab Chat генерише јединствени идентификатор за сваку конверзацију на основу URL-а странице, DOM пута до елемента и изабраног опсега текста. Можете га заменити прилагођеним `urlId`.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ово је корисно када структура ваших URL-ова може да се промијени али желите да задржите исте конверзације, или када желите да делите анотације преко више страница.

#### topBarTarget

Контролише приказ горње траке која показује број корисника и број дискусија. Поставите на `null` да у потпуности онемогућите горњу траку, или наведите DOM елемент у који ће бити приказана на одређеној локацији.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Онемогући горњу траку
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});

// Прикажи горњу траку у прилагођеној локацији
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('custom-header')
});
[inline-code-end]

#### hasDarkBackground

Омогућите тамни режим стилa када ваша страница има тамну позадину. Ово се детектује аутоматски, али можда ћете желети да га пренапишете.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функција повратног позива која се покреће кад год се број коментара промијени. Ово је корисно за ажурирање елемената UI-а попут значки или наслова странице.

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

Пошто Collab Chat проширује стандардни видгет за коментаре, можете користити било коју опцију конфигурације из основног FastComments видгета. Ево неких често коришћених опција:

#### locale

Подесите језик за UI видгета. FastComments подржава десетине језика.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Шпански
});
[inline-code-end]

#### readonly

Учините све конверзације само за читање. Корисници могу прегледати постојеће анотације, али не могу креирати нове или одговарати.

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

Погледајте SSO документацију за детаљне информације о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа угнежђавања одговора је дозвољено. По подразумевању, Collab Chat поставља ово на 0, што значи да су сви коментари равни (без угнежђених одговора). Можете ово промијенити ако желите нитоване конверзације.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволи 3 нивоа угнежђавања
});
[inline-code-end]

### Интерна конфигурација

Ове опције аутоматски поставља Collab Chat и не би требало да их мењате:

The `productId` is automatically set to `3` for Collab Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly.

### Комплетан примјер

Ево примера који показује више опција конфигурације заједно:

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

За комплетну листу свих доступних опција конфигурације наслеђених од основног видгета, видите главну FastComments документацију о конфигурацији.