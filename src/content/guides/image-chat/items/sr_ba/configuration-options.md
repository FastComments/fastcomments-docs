### Преглед

FastComments Image Chat проширује стандардни FastComments коментаторски видгет, па насљеђује све опције конфигурације из базног видгета док додаје неколико које су специфичне за анотације слика.

### Потребна конфигурација

#### tenantId

Ваш FastComments Tenant ID је обавезан. Можете га пронаћи у вашем [FastComments контролној табли](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Опције специфичне за Image Chat

#### urlId

По подразумевању, Image Chat генерише јединствени идентификатор за сваки разговор на основу URL-а странице, извора слике и X/Y координата. Ово можете замјенити прилагођеним `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Ово је корисно када структура вашег URL-а може да се мијења али желите задржати исте разговоре, или када желите дијелити анотације преко више страница.

#### chatSquarePercentage

Контролише величину кликабилних маркера за разговор као процент ширине слике. Подразумевано је 5%, што значи да је сваки маркер 5% ширине слике.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Veći, vidljiviji markeri
});
```

Мање вриједности креирају мање упадљиве маркере који боље раде за детаљне слике. Веће вриједности чине маркере лакшим за видјети и кликање на густим сликама или за кориснике на мобилним уређајима.

#### hasDarkBackground

Омогућите тамни режим стилова када ваша страница има тамну позадину.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функција повратног позива која се покреће кад год се број коментара промijени. Ово је корисно за ажурирање UI елемената попут значки или наслова странице.

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

Пошто Image Chat проширује стандардни коментаторски видгет, можете користити било коју опцију конфигурације из базног FastComments видгета. Ево неколико често кориштених опција:

#### locale

Подесите језик за UI видгета. FastComments подржава десетине језика.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Španski
});
```

#### readonly

Учините све разговоре само за читање. Корисници могу видјети постојеће маркере и дискусије али не могу креирати нове или одговарати.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso i simpleSSO

Интегришите са вашим системом аутентификације користећи Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Konfiguracija SSO
    }
});
```

Погледајте документацију о SSO за пунa детаље о опцијама аутентификације.

#### maxReplyDepth

Контролишите колико нивоа дубоко одговори могу ићи. По подразумевању, Image Chat поставља ово на 0, што значи да су сви коментари равни (без угнежђених одговора). Ово можете промијенити ако желите темељне разговоре.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Dozvoli 3 nivoa ugnježđivanja
});
```

### Интерна конфигурација

Ове опције аутоматски поставља Image Chat и не би требало да се надјеђују:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Флексибилност циљног елемента

Први параметар за `FastCommentsImageChat` може бити или `<img>` елемент директно или контејнерски елемент са сликом унутра:

```javascript
// Direktan <img> element
FastCommentsImageChat(document.getElementById('my-image'), config);

// Kontejner sa slikom unutra
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Видгет ће аутоматски пронаћи слику ако прослиједите контејнерски елемент.

### Комплетан примјер

Ево примјера који показује више опција конфигурације заједно:

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

За потпуни списак свих доступних опција конфигурације које насљеђује базни видгет, погледајте главну FastComments документацију о конфигурацији.