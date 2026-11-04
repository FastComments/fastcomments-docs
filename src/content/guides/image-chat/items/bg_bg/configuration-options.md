### Общ преглед

FastComments Image Chat разширява стандартния коментарен уиджет на FastComments, така че наследява всички конфигурационни опции от базовия уиджет, като добавя няколко специфични за анотации върху изображения.

### Задължителна конфигурация

#### tenantId

Изисква се вашият FastComments Tenant ID. Можете да го намерите в [FastComments таблото си](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Специфични опции за Image Chat

#### urlId

По подразбиране Image Chat генерира уникален идентификатор за всяка беседа въз основа на URL адреса на страницата, източника на изображението и X/Y координатите. Можете да го презапишете с персонализиран `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Това е полезно, когато структурата на вашия URL може да се промени, но искате да запазите същите беседа, или когато искате да споделите анотации между няколко страници.

#### chatSquarePercentage

Контролира размера на кликаемите маркери за чат като процент от ширината на изображението. По подразбиране е 5%, което означава, че всеки маркер е 5% от ширината на изображението.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // По-големи, по-видими маркери
});
```

По-малките стойности създават по-дискретни маркери, които работят по-добре за детайлни изображения. По-големите стойности правят маркерите по-лесни за виждане и кликване на натоварени изображения или за потребители на мобилни устройства.

#### hasDarkBackground

Активирайте стилизиране в тъмен режим, когато страницата ви има тъмен фон.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функция обратна връзка, която се извиква при всяка промяна на броя на коментарите. Полезна е за обновяване на UI елементи като баджети или заглавия на страници.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Наследени конфигурационни опции

Тъй като Image Chat разширява стандартния коментарен уиджет, можете да използвате всяка конфигурационна опция от базовия FastComments уиджет. Ето някои често използвани опции:

#### locale

Задайте езика за UI на уиджета. FastComments поддържа десетки езици.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Испански
});
```

#### readonly

Направете всички беседа само за четене. Потребителите могат да виждат съществуващите маркери и дискусии, но не могат да създават нови или да отговарят.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso и simpleSSO

Интегрирайте се със вашата система за автентикация чрез Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // SSO конфигурация
    }
});
```

Вижте SSO документацията за пълни подробности относно опциите за автентикация.

#### maxReplyDepth

Контролира на колко нива дълбоко могат да отиват отговорите. По подразбиране Image Chat задава това на 0, което означава, че всички коментари са плоски (без вложени отговори). Можете да промените това, ако искате беседа с разклонения.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Позволява 3 нива на влагане
});
```

### Вътрешна конфигурация

Тези опции се задават автоматично от Image Chat и не бива да бъдат презаписвани:

`productId` се задава автоматично на `2` за Image Chat. Разширението `floating-chat` се зарежда автоматично, за да осигури функционалността на прозореца за чат. Уиджетът автоматично разпознава мобилни устройства (екрани под 768px ширина) и съответно адаптира UI с чат прозорци на цял екран.

### Гъвкавост на целевия елемент

Първият параметър на `FastCommentsImageChat` може да бъде директно `<img>` елемент или контейнерен елемент с изображение вътре:

```javascript
// Директен елемент с изображение
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контейнер с изображение вътре
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Уиджетът ще намери изображението автоматично, ако подадете контейнерен елемент.

### Пълен пример

Ето пример, показващ няколко конфигурационни опции заедно:

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
        // Ваша SSO конфигурация
    },
    maxReplyDepth: 1
});
```

За пълен списък на всички налични конфигурационни опции, наследени от базовия уиджет, вижте основната документация за конфигурация на FastComments.