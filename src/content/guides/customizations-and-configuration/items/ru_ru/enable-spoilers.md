[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Мы можем включить поддержку спойлеров, установив флаг **enableSpoilers** в true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета выберите опцию «Включить спойлеры».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Когда текст выделен, и становится видна кнопка `SPOILER`, при нажатии текст будет скрыт до тех пор, пока пользователь не наведёт на него курсор мыши. В тёмном режиме мы делаем то же самое, но с другими
цветами, которые лучше подходят для тёмной темы.

Это также совместимо с WYSIWYG-редактором.