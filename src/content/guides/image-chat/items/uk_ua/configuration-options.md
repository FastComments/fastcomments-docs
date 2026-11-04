### Огляд

FastComments Image Chat розширює стандартний віджет коментування FastComments, тому він успадковує всі параметри конфігурації базового віджета, додаючи кілька специфічних для анотацій зображень.

### Необхідна конфігурація

#### tenantId

Потрібен ваш FastComments Tenant ID. Ви можете знайти його у своїй [панелі керування FastComments](https://fastcomments.com/auth/my-account/api-secret).

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo'
});
```

### Спеціальні параметри Image Chat

#### urlId

За замовчуванням Image Chat генерує унікальний ідентифікатор для кожної бесіди на основі URL сторінки, джерела зображення та координат X/Y. Ви можете перевизначити це власним `urlId`.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    urlId: 'my-custom-image-id'
});
```

Це корисно, коли структура ваших URL може змінюватися, але ви хочете зберегти ті самі бесіди, або коли хочете ділитися анотаціями між кількома сторінками.

#### chatSquarePercentage

Керує розміром клікабельних маркерів чату у відсотках від ширини зображення. За замовчуванням — 5%, тобто кожен маркер становить 5% ширини зображення.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 8  // Більші, помітніші маркери
});
```

Менші значення створюють менш нав'язливі маркери, які краще підходять для детальних зображень. Більші значення роблять маркери легшими для побачення та кліку на насичених зображеннях або для користувачів мобільних пристроїв.

#### hasDarkBackground

Увімкніть стилізацію темного режиму, якщо ваша сторінка має темний фон.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

#### commentCountUpdated

Функція зворотного виклику, яка спрацьовує щоразу, коли змінюється кількість коментарів. Це корисно для оновлення елементів інтерфейсу, як-от значки чи заголовки сторінок.

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

Оскільки Image Chat розширює стандартний віджет коментування, ви можете використовувати будь-який параметр конфігурації базового віджета FastComments. Ось деякі часто використовувані параметри:

#### locale

Встановіть мову інтерфейсу віджета. FastComments підтримує десятки мов.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'es'  // Іспанська
});
```

#### readonly

Зробіть усі бесіди доступними лише для читання. Користувачі можуть переглядати наявні маркери та обговорення, але не можуть створювати нові чи відповідати.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    readonly: true
});
```

#### sso та simpleSSO

Інтегруйтеся з вашою системою автентифікації за допомогою Single Sign-On.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    sso: {
        // Конфігурація SSO
    }
});
```

Дивіться документацію SSO для повних деталей щодо параметрів автентифікації.

#### maxReplyDepth

Керуйте тим, на скільки рівнів у глибину можуть йти відповіді. За замовчуванням Image Chat встановлює це значення на 0, тобто всі коментарі є плоскими (без вкладених відповідей). Ви можете змінити це, якщо хочете гіллясті бесіди.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    maxReplyDepth: 3  // Дозволити 3 рівні вкладеності
});
```

### Внутрішня конфігурація

Ці параметри встановлюються автоматично Image Chat і не повинні перевизначатися:

`productId` автоматично встановлюється на `2` для Image Chat. Розширення `floating-chat` автоматично завантажується для забезпечення функціональності вікна чату. Віджет автоматично визначає мобільні пристрої (екрани шириною менше 768px) і відповідно адаптує інтерфейс із повноекранними вікнами чату.

### Гнучкість цільового елемента

Перший параметр `FastCommentsImageChat` може бути або безпосередньо елементом `<img>`, або контейнерним елементом із зображенням всередині:

```javascript
// Безпосередньо елемент зображення
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

Для повного списку всіх доступних параметрів конфігурації, успадкованих від базового віджета, дивіться основну документацію конфігурації FastComments.