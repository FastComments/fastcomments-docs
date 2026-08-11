[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

За замовчуванням FastComments буде сортувати коментарі за напрямком сортування "Most Relevant".

Сортування Most Relevant враховує час залишення коментаря та кількість голосів при сортуванні.

Користувач може потім змінити напрямок сортування на "Oldest" або "Newest First" у інтерфейсі віджета коментарів.

Однак ми можемо змінити значення за замовчуванням на будь-яке з трьох. Наприклад, якщо ви хочете показати найстаріші коментарі першими:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Зміна сортування за замовчуванням на найстаріші спочатку'; code-example-end]

Ми встановлюємо значення **defaultSortDirection** на "OF", щоб задати напрямок "OF".

Для напрямку сортування newest-first ми б зробили наступне:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Зміна сортування за замовчуванням на найновіші спочатку'; code-example-end]

Допустимі значення для **defaultSortDirection** є:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Це також можна зробити без коду. На сторінці налаштування віджета, дивіться розділ "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Селектор "Default Sort Direction", що пропонує Most Relevant, Newest First та Oldest First'; title='Зміна напрямку сортування за замовчуванням' app-screenshot-end]

Зверніть увагу, що коментарі на кожній сторінці для кожного напрямку сортування попередньо обчислюються, тому всі напрямки сортування мають однакову продуктивність.