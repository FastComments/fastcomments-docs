### Преглед

FastComments Image Chat проширује стандардни FastComments видгет за коментаре, па наследи све опције конфигурације из основног видгета док додаје неколико опција специфичних за анотације слика.

### Потребна конфигурација

#### tenantId

Потребан је ваш FastComments Tenant ID. Можете га пронаћи у вашем [FastComments контролном панелу](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Опције специфичне за Image Chat

#### urlId

По подразумевaњу, Image Chat генерише јединствени идентификатор за сваку конверзацију на основу URL-а странице, извора слике и X/Y координата. Можете то прекрити прилагођеним `urlId`-ом.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ово је корисно када структура ваших URL-ова може да се промени али желите да задржите исте конверзације, или када желите да делите анотације преко више страница.

#### chatSquarePercentage

Контролише величину кликабилних ознака за ћаскање као проценат ширине слике. Подразумевана вредност је 5%, што значи да је свака ознака ширине 5% од ширине слике.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Већи, видљивији маркери
});
```

Мање вредности праве мање инвазивне ознаке које боље функционишу за детаљне слике. Веће вредности чине ознаке лакшим за виђење и кликање на сликама пуним детаља или за кориснике на мобилним уређајима.

#### hasDarkBackground

Омогућите дизајн у темном режиму када ваша страница има тамну позадину.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функција повратног позива која се покреће кад год се број коментара промени. Ово је корисно за ажурирање елемената корисничког интерфејса као што су значке или наслови странице.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Наслеђене опције конфигурације

Пошто Image Chat проширује стандардни видгет за коментаре, можете користити било коју опцију конфигурације из основног FastComments видгета. Ево неких често коришћених опција:

#### locale

Подесите језик корисничког интерфејса видгета. FastComments подржава десетине језика.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Шпански
});
```

#### readonly

Учини све конверзације само за читање. Корисници могу да прегледају постојеће ознаке и дискусије али не могу да креирају нове или одговарају.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Интегришите са вашим системом аутентификације користећи Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Конфигурација SSO
    }
});
```

Погледајте документацију за SSO за детаљна упутства о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа дубоко одговори могу да иду. По подразумевaњу, Image Chat подешава ово на 0, што значи да су сви коментари равни (без угнеждених одговора). Можете то променити ако желите нитирану дискусију.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволи 3 нивоа угнеждених одговора
});
```

### Интерна конфигурација

Ове опције аутоматски подешава Image Chat и не би требало да их мењате:

Параметар `productId` се аутоматски поставља на `2` за Image Chat. `floating-chat` екстензија се аутоматски учитава да обезбеди функционалност прозора за ћаскање. Видгет аутоматски детектује мобилне уређаје (екрани ширине мање од 768px) и прилагођава кориснички интерфејс у складу с тим, са ћаскањем преко целог екрана.

### Флексибилност циљног елемента

Први параметар функције `FastCommentsImageChat` може бити директно `<img>` елемент или контејнер елемент који у себи садржи слику:

```javascript
// Директан <img> елемент
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контејнер са сликом унутра
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Видгет ће аутоматски пронаћи слику ако проследите контејнер елемент.

### Комплетан пример

Ево примера који показује више опција конфигурације заједно:

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

За комплетан списак свих доступних опција конфигурације које се наслеђују из основног видгета, погледајте главну FastComments документацију о конфигурацији.