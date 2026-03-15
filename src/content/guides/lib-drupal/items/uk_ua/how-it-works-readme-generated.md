Коли користувач відвідує сутність з увімкненим полем FastComments:

1. JavaScript-виджет FastComments завантажується з CDN.
2. Якщо налаштовано SSO, ідентичність користувача Drupal передається в FastComments.
3. Резервний механізм `<noscript>` забезпечує серверно-згенеровані коментарі для користувачів без JavaScript (лише в режимах Live Comments та Streaming Chat).