---
[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

За замовчуванням форматування в FastComments здійснюється шляхом додавання навколо тексту видимих анкерних тегів, таких як `<b></b>`. Клацання по панелі інструментів або використання гарячих клавіш робить це за вас. Однак деякі спільноти можуть захотіти ввімкнути форматування без видимих анкерних тегів. Це називається включенням WYSIWYG (те, що бачиш — те й отримуєш) редактора. Цей редактор виглядає точно так само, як і стандартний, за винятком того, що він завантажує додатковий код, який дозволяє користувачам робити текст жирним, підкресленим тощо без видимих анкерних тегів.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета знайдіть опцію "Увімкнути розширене форматування".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]

---