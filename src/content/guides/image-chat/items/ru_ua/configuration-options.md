### Overview

FastComments Image Chat расширяет стандартный виджет комментариев FastComments, поэтому он наследует все параметры конфигурации от базового виджета и добавляет несколько, специфичных для аннотаций изображений.

### Required Configuration

#### tenantId

Требуется ваш FastComments Tenant ID. Вы можете найти его в вашей [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Image Chat Specific Options

#### urlId

По умолчанию Image Chat генерирует уникальный идентификатор для каждого разговора на основе URL страницы, источника изображения и координат X/Y. Вы можете переопределить это, указав кастомный `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Это полезно, когда структура ваших URL может измениться, но вы хотите сохранить те же беседы, или когда хотите делиться аннотациями между несколькими страницами.

#### chatSquarePercentage

Управляет размером кликабельных маркеров чата в процентах от ширины изображения. По умолчанию 5%, то есть каждый маркер занимает 5% ширины изображения.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Больше, более заметные маркеры
});
```

Меньшие значения создают менее навязчивые маркеры, которые лучше подходят для детализированных изображений. Большие значения делают маркеры проще увидеть и нажать на загруженных изображениях или для пользователей на мобильных устройствах.

#### hasDarkBackground

Включите стили для тёмного режима, если на вашей странице тёмный фон.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функция-обратного вызова, которая срабатывает при каждом изменении количества комментариев. Это полезно для обновления элементов интерфейса, таких как бейджи или заголовки страницы.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Inherited Configuration Options

Поскольку Image Chat расширяет стандартный виджет комментариев, вы можете использовать любые параметры конфигурации из базового виджета FastComments. Ниже приведены некоторые часто используемые опции:

#### locale

Установите язык для интерфейса виджета. FastComments поддерживает десятки языков.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
```

#### readonly

Сделать все беседы доступными только для чтения. Пользователи смогут просматривать существующие маркеры и обсуждения, но не смогут создавать новые или отвечать.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Интегрируйте с вашей системой аутентификации, используя Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Конфигурация SSO
    }
});
```

Смотрите документацию по SSO для полного описания вариантов аутентификации.

#### maxReplyDepth

Контролируйте, насколько глубоко могут быть вложены ответы. По умолчанию Image Chat устанавливает это значение в 0, что означает, что все комментарии плоские (без вложенных ответов). Вы можете изменить это, если хотите иметь древовидные беседы.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Разрешить 3 уровня вложенности
});
```

### Internal Configuration

Эти параметры автоматически задаются Image Chat и не должны переопределяться:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Target Element Flexibility

Первый параметр для `FastCommentsImageChat` может быть либо элементом `<img>` напрямую, либо контейнером с изображением внутри:

```javascript
// Прямой элемент изображения
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контейнер с изображением внутри
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Виджет автоматически найдёт изображение, если вы передадите элемент-контейнер.

### Complete Example

Ниже пример, показывающий несколько опций конфигурации вместе:

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

Для полного списка всех доступных параметров конфигурации, унаследованных от базового виджета, смотрите основную документацию по конфигурации FastComments.