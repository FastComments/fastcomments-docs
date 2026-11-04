### Преглед

FastComments Collab Chat проширује стандардни FastComments виџет за коментаре, тако да наслеђује све опције конфигурације основног виџета, уз додатак неколико опција специфичних за текстуалне анотације.

### Обавезна конфигурација

#### tenantId

Потребан је ваш FastComments Tenant ID. Можете га пронаћи у свом [FastComments контролном панелу](https://fastcomments.com/auth/my-account/api-secret).

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo'
});
[inline-code-end]

### Опције специфичне за Collab Chat

#### urlId

Подразумевано, Collab Chat генерише јединствени идентификатор за сваки разговор на основу URL адресе странице, DOM путање до елемента и изабраног опсега текста. Можете то заменити прилагођеним `urlId`.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    urlId: 'my-custom-page-id'
});
[inline-code-end]

Ово је корисно када се структура ваше URL адресе може променити, али желите да задржите исте разговоре, или када желите да делите анотације на више страница.

#### topBarTarget

Контролише приказ горње траке која приказује број корисника и број дискусија. Поставите на `null` да бисте потпуно онемогућили горњу траку, или наведите DOM елемент да бисте је приказали на одређеној локацији.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
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

Омогућите стилизовање тамног режима када ваша страница има тамну позадину. Ова детекција је аутоматска, али може бити пожељно да је замените.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    hasDarkBackground: true
});
[inline-code-end]

#### commentCountUpdated

Функција повратног позива која се активира сваки пут када се број коментара промени. Ово је корисно за ажурирање UI елемената као што су беџеви или наслови страница.

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

Пошто Collab Chat проширује стандардни виџет за коментаре, можете користити било коју опцију конфигурације основног FastComments виџета. Ево неких често коришћених опција:

#### locale

Поставите језик за UI виџета. FastComments подржава десетине језика.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
[inline-code-end]

#### readonly

Учините све разговоре само за читање. Корисници могу да виде постојеће анотације, али не могу да креирају нове нити да одговарају.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    readonly: true
});
[inline-code-end]

#### sso и simpleSSO

Интегришите се са својим системом аутентификације коришћењем Single Sign-On-а.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    sso: {
        // SSO configuration
    }
});
[inline-code-end]

Погледајте SSO документацију за све детаље о опцијама аутентификације.

#### maxReplyDepth

Контролишите на колико нивоа дубоко могу ићи одговори. Подразумевано, Collab Chat поставља ово на 0, што значи да су сви коментари равни (без угнежђених одговора). Можете ово променити ако желите разговоре са гранањем.

[inline-code-attrs-start title = "Пример конфигурације"; type = "javascript"; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Allow 3 levels of nesting
});
[inline-code-end]

### Унутрашња конфигурација

Ове опције аутоматски поставља Collab Chat и не би требало да буду замењене:

`productId` је аутоматски постављен на `3` за Collab Chat. `floating-chat` проширење се аутоматски учитава да би обезбедило функционалност прозора за ћаскање. Виџет аутоматски детектује мобилне уређаје (екране ширине мање од 768px) и сходно томе прилагођава UI.

### Комплетан пример

Ево примера који приказује више опција конфигурације заједно:

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
        // Your SSO config
    },
    maxReplyDepth: 1
});
[inline-code-end]

За комплетну листу свих доступних опција конфигурације наслеђених из основног виџета, погледајте главну документацију за конфигурацију FastComments-а.