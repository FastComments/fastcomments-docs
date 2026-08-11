[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

За замовчуванням функції форматування в FastComments здійснюються шляхом додавання видимих тегів-якорів, таких як `<b></b>`, навколо вашого тексту. Натискання на панель інструментів
або використання швидких клавіш робить це за вас. Однак деякі спільноти можуть захотіти використовувати форматування без тегів-якорів. Це називається увімкненням
редактора WYSIWYG (what you see is what you get). Цей редактор виглядає точно так само, як і стандартний, за винятком того, що він завантажує додатковий
код, який дозволяє користувачам робити текст жирним, підкресленим тощо без видимих тегів-якорів.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Увімкнення редагування WYSIWYG'; code-example-end]

Це також можна зробити без коду. На сторінці налаштування віджета перегляньте параметр "Enable Advanced Formatting" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Сторінка налаштування віджета з позначеним прапорцем Enable Advanced Formatting для ввімкнення редактора WYSIWYG'; title='Увімкнути WYSIWYG' app-screenshot-end]

---