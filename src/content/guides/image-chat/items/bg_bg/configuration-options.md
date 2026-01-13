### Преглед

FastComments Image Chat разширява стандартната джаджа за коментари на FastComments, така че наследява всички опции за конфигуриране от базовата джаджа, като добавя няколко, специфични за анотации върху изображения.

### Задължителна конфигурация

#### tenantId

Необходими е вашият FastComments Tenant ID. Можете да го намерите в [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Опции, специфични за Image Chat

#### urlId

По подразбиране Image Chat генерира уникален идентификатор за всеки разговор въз основа на URL адреса на страницата, източника на изображението и координатите X/Y. Можете да презапишете това с потребителски `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Това е полезно, когато структурата на вашите URL адреси може да се промени, но искате да запазите същите разговори, или когато искате да споделяте анотации между няколко страници.

#### chatSquarePercentage

Контролира размера на кликаемите маркери за чат като процент от ширината на изображението. По подразбиране е 5%, което означава, че всеки маркер е 5% от ширината на изображението.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // По-големи, по-видими маркери
});
```

По-малките стойности създават по-малко натрапчиви маркери, които работят по-добре за детайлни изображения. По-големите стойности правят маркерите по-лесни за виждане и кликване при натоварени изображения или за потребители на мобилни устройства.

#### hasDarkBackground

Активирайте стилизиране в тъмен режим, когато вашата страница има тъмен фон.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функция за обратно извикване (callback), която се изпълнява всеки път, когато броят коментари се промени. Това е полезно за обновяване на UI елементи като значки или заглавия на страницата.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Наследени опции за конфигурация

Тъй като Image Chat разширява стандартната джаджа за коментари, можете да използвате всяка опция за конфигуриране от базовата FastComments джаджа. Ето някои често използвани опции:

#### locale

Задава езика за потребителския интерфейс на джаджата. FastComments поддържа десетки езици.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Испански
});
```

#### readonly

Направете всички разговори достъпни само за четене. Потребителите могат да виждат съществуващите маркери и дискусии, но не могат да създават нови или да отговарят.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Integrate with your authentication system using Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Конфигурация на SSO
    }
});
```

Вижте документацията за SSO за пълни подробности относно опциите за удостоверяване.

#### maxReplyDepth

Контролира колко нива дълбоко могат да бъдат отговорите. По подразбиране Image Chat задава това на 0, което означава, че всички коментари са плоски (без вложени отговори). Можете да промените това, ако искате нишки от разговори.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Позволява 3 нива на вложеност
});
```

### Вътрешна конфигурация

Тези опции се задават автоматично от Image Chat и не трябва да бъдат презаписвани:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Гъвкавост на целевия елемент

Първият параметър на `FastCommentsImageChat` може да бъде или елемент `<img>` директно, или контейнерен елемент с изображение вътре:

```javascript
// Директен елемент на изображението
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контейнер с изображение вътре
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Джаджата ще намери изображението автоматично, ако преминете контейнерен елемент.

### Пълен пример

Ето пример, показващ няколко опции за конфигуриране заедно:

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
        // Вашата конфигурация за SSO
    },
    maxReplyDepth: 1
});
```

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.