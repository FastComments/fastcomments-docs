[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

За замовчуванням FastComments сортує коментарі за напрямком сортування «Most Relevant».

Сортування «Most Relevant» враховує час залишення коментаря та кількість голосів при сортуванні.

Користувач може змінити напрям сортування на «Oldest» або «Newest First» у інтерфейсі віджета коментарів.

Однак ми можемо змінити значення за замовчуванням на будь-який з трьох. Наприклад, якщо ви хочете показувати найстаріші коментарі першими:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

Ми встановлюємо значення **defaultSortDirection** в "OF", щоб встановити напрямок "OF".

Для напрямку сортування «Newest First» ми зробили б таке:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

Допустимі значення для **defaultSortDirection**:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

Це також можна зробити без коду. На сторінці налаштування віджета див. розділ "Default Sort Direction".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Зверніть увагу, що коментарі на кожній сторінці для кожного напрямку сортування попередньо обчислюються, тому всі напрямки сортування мають однакову продуктивність.