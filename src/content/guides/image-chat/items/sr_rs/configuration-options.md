### Преглед

FastComments Image Chat проширује стандардни FastComments видгет за коментаре, тако да наследи све опције конфигурације из основног видгета уз додавање неколико специфичних за анотације слика.

### Потребна конфигурација

#### tenantId

Вам је потребан ваш FastComments Tenant ID. Можете га пронаћи у вашем [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Опције специфичне за Image Chat

#### urlId

По подразумевaњу, Image Chat генерише јединствeни идентификатор за сваки разговор на основу URL странице, извора слике и X/Y координата. Можете то преоптеретити прилагођеним `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ово је корисно када структура вашег URL-а може да се промени, али желите да задржите исте разговоре, или када желите да делите анотације преко више страница.

#### chatSquarePercentage

Контролише величину кликабилних маркера за ћаскање као проценат ширине слике. Подразумевана вредност је 5%, што значи да је сваки маркер 5% ширине слике.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Веће, видљивије ознаке
});
```

Мање вредности праве мање инвазивне ознаке које боље функционишу за детаљне слике. Веће вредности чине ознаке лакшим за видети и клик на ужурбаним сликама или за кориснике на мобилним уређајима.

#### hasDarkBackground

Омогућите стил такође за мрачни режим када ваша страница има тамну позадину.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функција повратног позива која се позива кад год се број коментара промени. Ово је корисно за ажурирање елемената корисничког интерфејса као што су значке или наслови страница.

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

Поставите језик за кориснички интерфејс видгета. FastComments подржава десетине језика.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Шпански
});
```

#### readonly

Учини све разговоре само за читање. Корисници могу да прегледају постојеће маркере и дискусије, али не могу да креирају нове или одговарају.

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
        // Конфигурација SSO-а
    }
});
```

Погледајте SSO документацију за комплетне детаље о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа дубоко одговори могу да иду. По подразумевaњу, Image Chat поставља ово на 0, што значи да су сви коментари равни (без унутрашњег улажења). Можете то променити ако желите нитоване разговоре.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволите 3 нивоа улажења
});
```

### Интерна конфигурација

Ове опције аутоматски поставља Image Chat и не треба их преоптерећивати:

Параметар `productId` се аутоматски поставља на `2` за Image Chat. Екстензија `floating-chat` се аутоматски учитава да би обезбедила функционалност прозора за ћаскање. Видгет аутоматски детектује мобилне уређаје (екрани ширине мање од 768px) и прилагођава кориснички интерфејс у складу са тим, са режимом за цео екран за прозоре ћаскања.

### Флексибилност циљног елемента

Први параметар за `FastCommentsImageChat` може бити или `<img>` елемент директно или контејнер елемент који у себи садржи слику:

```javascript
// Директан <img> елемент
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контејнер са сликом унутар
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

За комплетну листу свих доступних опција конфигурације које се наслеђују из основног видгета, погледајте главну FastComments документацију о конфигурацији.