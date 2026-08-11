[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Мы можем включить поддержку спойлеров, установив флаг **enableSpoilers** в значение true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Включение спойлеров'; code-example-end]

Это также можно сделать без кода. На странице настройки виджета найдите опцию «Enable Spoilers».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Страница настройки виджета с отмеченным флажком «Enable Spoilers», чтобы добавить кнопку SPOILER в редактор'; title='Включить спойлеры' app-screenshot-end]

Когда текст выделен, и теперь видимая кнопка `SPOILER` нажата, текст будет скрыт до тех пор, пока пользователь не наведёт на него курсор. Для тёмного режима мы делаем то же самое, но с другими
цветами, которые лучше подходят для тёмного режима.

Это также совместимо с редактором WYSIWYG.