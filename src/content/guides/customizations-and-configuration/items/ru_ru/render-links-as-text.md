По умолчанию FastComments отображает ссылки так: [https://exmaple.com](https://exmaple.com) - где URL ссылки становится кликабельной
HTML-ссылкой.

Некоторые сайты могут захотеть отключить это, например, чтобы отпугнуть мошенников. Мы предоставляем эту возможность, установив `Comment HTML Rendering Option` в значение `Links as Text`.

Это можно настроить без кода, на странице настройки виджета, для всего домена, или страницы:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; title='Render Links as Text' app-screenshot-end]