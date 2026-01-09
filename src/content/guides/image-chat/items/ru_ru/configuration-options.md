---
### Обзор

FastComments Image Chat расширяет стандартный виджет комментариев FastComments, поэтому он наследует все параметры конфигурации базового виджета и добавляет несколько, специфичных для аннотаций изображений.

### Обязательная конфигурация

#### tenantId

Требуется ваш FastComments Tenant ID. Вы можете найти его в вашей [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Параметры, специфичные для Image Chat

#### urlId

По умолчанию Image Chat генерирует уникальный идентификатор для каждого разговора на основе URL страницы, источника изображения и координат X/Y. Вы можете переопределить это с помощью пользовательского `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Это полезно, когда структура ваших URL может измениться, но вы хотите сохранить те же разговоры, или когда вы хотите делиться аннотациями на нескольких страницах.

#### chatSquarePercentage

Управляет размером кликабельных маркеров чата в процентах от ширины изображения. По умолчанию это 5%, то есть каждый маркер занимает 5% ширины изображения.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Более крупные, более заметные маркеры
});
```

Меньшие значения создают менее навязчивые маркеры, которые лучше подходят для детализированных изображений. Более крупные значения делают маркеры легче видимыми и нажатие по ним удобнее на загруженных изображениях или для пользователей на мобильных устройствах.

#### hasDarkBackground

Включает оформление в тёмном режиме, когда на вашей странице тёмный фон.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функция обратного вызова, которая выполняется при каждом изменении количества комментариев. Это полезно для обновления элементов интерфейса, таких как бейджи или заголовки страниц.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Унаследованные параметры конфигурации

Поскольку Image Chat расширяет стандартный виджет комментариев, вы можете использовать любой параметр конфигурации из базового виджета FastComments. Ниже приведены некоторые часто используемые параметры:

#### locale

Устанавливает язык интерфейса виджета. FastComments поддерживает десятки языков.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Испанский
});
```

#### readonly

Сделать все разговоры только для чтения. Пользователи могут просматривать существующие маркеры и обсуждения, но не могут создавать новые или отвечать.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso и simpleSSO

Интеграция с вашей системой аутентификации с использованием Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Конфигурация SSO
    }
});
```

См. документацию по SSO для полного описания параметров аутентификации.

#### maxReplyDepth

Контролирует, насколько глубоко могут быть вложены ответы. По умолчанию Image Chat устанавливает это значение в 0, что означает, что все комментарии плоские (без вложенных ответов). Вы можете изменить это, если хотите организовать древовидные обсуждения.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Разрешить 3 уровня вложенности
});
```

### Внутренняя конфигурация

Эти параметры автоматически устанавливаются Image Chat и не должны переопределяться:

Значение `productId` автоматически устанавливается в `2` для Image Chat. Расширение `floating-chat` автоматически загружается для обеспечения функциональности окна чата. Виджет автоматически обнаруживает мобильные устройства (экраны шириной менее 768px) и соответственно подстраивает интерфейс с полноэкранными окнами чата.

### Гибкость целевого элемента

Первый параметр для `FastCommentsImageChat` может быть либо элементом `<img>` напрямую, либо контейнером с изображением внутри:

```javascript
// Прямой элемент <img>
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контейнер с вложенным изображением
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Виджет автоматически найдёт изображение, если вы передадите элемент-контейнер.

### Полный пример

Вот пример, показывающий несколько параметров конфигурации одновременно:

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
        // Ваша конфигурация SSO
    },
    maxReplyDepth: 1
});
```

Для полного списка всех доступных параметров конфигурации, унаследованных от базового виджета, см. основную документацию по конфигурации FastComments.

---