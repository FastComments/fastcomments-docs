[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Мы можем включить поддержку спойлеров, установив флаг **enableSpoilers** в значение true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Это можно сделать и без кода. На странице настройки виджета посмотрите опцию "Включить спойлеры".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Когда текст выделен, и нажата теперь видимая кнопка `SPOILER`, текст будет замаскирован до тех пор, пока пользователь не наведёт на него курсор. Для тёмного режима мы делаем то же самое, с другими
цветами, которые лучше соответствуют тёмному режиму.

Это также совместимо с WYSIWYG-редактором.

---