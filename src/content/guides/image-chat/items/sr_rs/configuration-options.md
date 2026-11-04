### Преглед

FastComments Image Chat проширује стандардни FastComments виџет за коментаре, тако да наслеђује све конфигурационе опције основног виџета, уз додатак неколико опција специфичних за анотације слика.

### Обавезна конфигурација

#### tenantId

Потребан је ваш FastComments Tenant ID. Можете га пронаћи на свом [FastComments контролном панелу](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Опције специфичне за Image Chat

#### urlId

Подразумевано, Image Chat генерише јединствени идентификатор за сваки разговор на основу URL адресе странице, извора слике и X/Y координата. Ово можете заменити прилагођеним `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ово је корисно када се структура ваше URL адресе може променити, али желите да задржите исте разговоре, или када желите да делите анотације на више страница.

#### chatSquarePercentage

Контролише величину кликабилних маркера за разговор као проценат ширине слике. Подразумевана вредност је 5%, што значи да је сваки маркер 5% ширине слике.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Већи, видљивији маркери
});
```

Мање вредности стварају мање упадљиве маркере који боље функционишу код детаљних слика. Веће вредности чине маркере лакшим за уочавање и кликање на прометним сликама или за кориснике на мобилним уређајима.

#### hasDarkBackground

Омогућите стилизовање тамног режима када ваша страница има тамну позадину.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функција повратног позива која се активира сваки пут када се број коментара промени. Ово је корисно за ажурирање UI елемената попут беџева или наслова странице.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Наслеђене конфигурационе опције

Пошто Image Chat проширује стандардни виџет за коментаре, можете користити било коју конфигурациону опцију основног FastComments виџета. Ево неких често коришћених опција:

#### locale

Поставите језик за UI виџета. FastComments подржава десетине језика.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Шпански
});
```

#### readonly

Учините све разговоре само за читање. Корисници могу да виде постојеће маркере и дискусије, али не могу да креирају нове нити да одговарају.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso и simpleSSO

Интегришите се са својим системом аутентификације коришћењем Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO конфигурација
    }
});
```

Погледајте SSO документацију за све детаље о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа дубоко одговори могу ићи. Подразумевано, Image Chat поставља ову вредност на 0, што значи да су сви коментари равни (без угњежђених одговора). Ово можете променити ако желите разговоре са нитима.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволи 3 нивоа угњежђивања
});
```

### Интерна конфигурација

Ове опције аутоматски поставља Image Chat и не би требало да буду замењене:

`productId` је аутоматски постављен на `2` за Image Chat. `floating-chat` екстензија се аутоматски учитава како би обезбедила функционалност прозора за разговор. Виџет аутоматски детектује мобилне уређаје (екране ширине мање од 768px) и прилагођава UI коришћењем прозора за разговор преко целог екрана.

### Флексибилност циљног елемента

Први параметар за `FastCommentsImageChat` може бити или директно `<img>` елемент или контејнер елемент са сликом унутра:

```javascript
// Директан елемент слике
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контејнер са сликом унутра
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Виџет ће аутоматски пронаћи слику ако проследите контејнер елемент.

### Комплетан пример

Ево примера који приказује више конфигурационих опција заједно:

```javascript
FastCommentsImageChat(document.getElementById('product-image'), {
    tenantId: 'demo',
    urlId: 'product-v2-main',
    chatSquarePercentage: 6,
    hasDarkBackground: false,
    locale: 'en',
    commentCountUpdated: function(count) {
        document.title = count > 0 ? `(${count}) Product Photo` : 'Product Photo';
    },
    sso: {
        // Ваша SSO конфигурација
    },
    maxReplyDepth: 1
});
```

За комплетну листу свих доступних конфигурационих опција наслеђених из основног виџета, погледајте главну документацију за конфигурацију FastComments.