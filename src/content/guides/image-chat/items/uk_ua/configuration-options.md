### Огляд

FastComments Image Chat розширює стандартний віджет коментарів FastComments, тому він наслідує всі параметри конфігурації базового віджета, додаючи кілька параметрів, специфічних для анотацій зображень.

### Необхідна конфігурація

#### tenantId

Ваш FastComments Tenant ID обов'язковий. Ви можете знайти його в [панелі керування FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Параметри, специфічні для Image Chat

#### urlId

За замовчуванням Image Chat генерує унікальний ідентифікатор для кожної розмови на основі URL сторінки, джерела зображення та координат X/Y. Ви можете перевизначити це користувацьким `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Це корисно, коли структура ваших URL може змінитися, але ви хочете зберегти ті самі розмови, або коли ви хочете ділитися анотаціями між кількома сторінками.

#### chatSquarePercentage

Керує розміром клікабельних маркерів чату як відсоток від ширини зображення. За замовчуванням це 5%, тобто кожен маркер займає 5% ширини зображення.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Більші, більш помітні маркери
});
```

Менші значення створюють менш нав'язливі маркери, що краще підходить для деталізованих зображень. Більші значення роблять маркери легшими для помітності та натискання на завантажених зображеннях або для користувачів на мобільних пристроях.

#### hasDarkBackground

Увімкніть оформлення для темного режиму, коли на вашій сторінці темний фон.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функція зворотного виклику, яка виконується щоразу, коли змінюється кількість коментарів. Це корисно для оновлення елементів інтерфейсу, таких як бейджі або заголовки сторінок.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    commentCountUpdated: function(count) {
        console.log('Total comments:', count);
        document.getElementById('badge').textContent = count;
    }
});
```

### Успадковані параметри конфігурації

Оскільки Image Chat розширює стандартний віджет коментарів, ви можете використовувати будь-яку опцію конфігурації з базового віджета FastComments. Ось деякі часто використовувані опції:

#### locale

Встановіть мову для інтерфейсу віджета. FastComments підтримує десятки мов.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Spanish
});
```

#### readonly

Зробіть всі розмови доступними тільки для читання. Користувачі можуть переглядати існуючі маркери та обговорення, але не можуть створювати нові або відповідати.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso and simpleSSO

Інтегруйте з вашою системою автентифікації за допомогою Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Налаштування SSO
    }
});
```

Див. документацію SSO для повних відомостей про параметри автентифікації.

#### maxReplyDepth

Керуйте тим, наскільки глибоко можуть вкладатися відповіді. За замовчуванням Image Chat встановлює це в 0, тобто всі коментарі плоскі (без вкладених відповідей). Ви можете змінити це, якщо хочете ниткоподібні (threaded) розмови.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволити 3 рівні вкладеності
});
```

### Внутрішня конфігурація

Ці параметри автоматично встановлюються Image Chat і їх не слід перевизначати:

The `productId` is automatically set to `2` for Image Chat. The `floating-chat` extension is automatically loaded to provide the chat window functionality. The widget automatically detects mobile devices (screens under 768px wide) and adjusts the UI accordingly with fullscreen chat windows.

### Гнучкість цільового елемента

Першим параметром для `FastCommentsImageChat` може бути або елемент `<img>` безпосередньо, або контейнерний елемент з зображенням всередині:

```javascript
// Прямий елемент зображення
FastCommentsImageChat(document.getElementById('my-image'), config);

// Контейнер із зображенням всередині
FastCommentsImageChat(document.querySelector('.image-wrapper'), config);
```

Віджет автоматично знайде зображення, якщо ви передасте контейнерний елемент.

### Повний приклад

Ось приклад, що показує кілька параметрів конфігурації разом:

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
        // Ваша конфігурація SSO
    },
    maxReplyDepth: 1
});
```

For a complete list of all available configuration options inherited from the base widget, see the main FastComments configuration documentation.