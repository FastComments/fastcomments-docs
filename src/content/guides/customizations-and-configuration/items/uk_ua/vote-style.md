[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

За замовчуванням FastComments відображатиме опції голосування у вигляді стрілок вгору та вниз, дозволяючи користувачам голосувати за або проти коментаря.

Проте можна змінити стиль панелі голосування. Наразі доступні стандартні кнопки Вгору/Вниз або використання механізму голосування в стилі серця.

Ми використовуємо прапорець **voteStyle** таким чином:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Ми настійно радимо зробити це без коду, оскільки це також включає серверні валідації. На сторінці налаштування віджета див. розділ «Стиль голосування».

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Голосування також можна вимкнути — див. `Disable Voting` вище за опціями стилю.