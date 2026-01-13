[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments створено з урахуванням можливості кастомізації. Віджет коментарів сам запускається всередині iframe з міркувань безпеки, тому для застосування
власних стилів потрібно скористатися одним із двох підходів.

Перший, найпростіший підхід, і той, який ми надаємо перевагу, — це використання [сторінки налаштування віджета](https://fastcomments.com/auth/my-account/customize-widget).

На сторінці налаштування віджета перегляньте розділ "Show Advanced Options", у якому є область з позначкою "Custom CSS":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Цей підхід має кілька переваг:
1. Введений CSS мініфікується перед відправленням користувачеві, а форматування зберігається послідовним у інтерфейсі редагування.
2. Ви отримуєте всі переваги інтерфейсу налаштування віджета, наприклад, просте налаштування віджета коментарів по-іншому для різних сайтів.
3. Коли ми вносимо зміни до віджета коментарів, ваші власні стилі будуть протестовані в рамках нашого процесу випуску.

Другий підхід — вказати параметр **customCSS** у конфігурації віджета, ось так:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Однак це має *обмеження*:
1. Існує обмеження на обсяг кастомного CSS, який можна передати, перш ніж наші сервери відхилять запит через розмір заголовків.
2. Ви маєте керувати кастомним CSS у своїй інфраструктурі та системі збірки. Це також може бути перевагою, а не недоліком.
3. Існує додаткове накладне передавання кастомного CSS по мережі **двічі** в цьому випадку: його необхідно відправити на наші сервери, а потім повернути вмістом iframe. Проте для більшості обсягів даних це непомітно.
4. Поширена оптимізація — мініфікувати CSS, щоб зменшити його розмір при передаванні по мережі; проте з таким підходом вам доведеться робити це самостійно.
5. Ваш кастомний CSS не буде протестований, коли ми вносимо зміни.

### External CSS Files

You can tell the widget to fetch an external file by using `@import`!

Рекомендується розміщувати `@import` у правилі кастомізації. Таким чином, якщо нам колись знадобиться внести зміни до віджета коментарів, ми зможемо використовувати наші автоматизовані
інструменти для перевірки вашої конфігурації. Тобто, наприклад, ви створите правило кастомізації в інтерфейсі налаштування віджета, натиснете `Advanced` і введете в `Custom CSS`:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

You can also load an external CSS file via the `customCSS` property:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

However, remember that your CSS won't be able to be tested by us if you do this. 

### User Profile Modal Styling

Модальні вікна профілю користувача також можна стилізувати за допомогою кастомного CSS. Проте щоб переконатися, що кастомні стилі застосовуються до профілів користувачів, усі CSS-селектори повинні мати префікс `.user-profile`. Без цього префіксу кастомні стилі будуть ігноруватися для модальних вікон профілю користувача.

For example:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

У FastComments ми знаємо, що наші клієнти налаштовують віджет коментарів. Це задумано спеціально — останнє, чого ми хочемо, щоб наш продукт спричиняв дизайн
невідповідності у вашому продукті.

Оскільки це важлива частина нашого продукту, у нас є конвеєр збірки, який дозволяє нам переглядати зміни у віджеті коментарів для кожного клієнта під час кожного випуску.

If we find minor issues, we will update your account to ensure our release goes smoothly. If we see major breaking changes, this allows us to halt the release.

---