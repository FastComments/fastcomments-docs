[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

За замовчуванням FastComments відображає варіанти голосування у вигляді стрілок вгору та вниз, дозволяючи користувачам підвищувати або знижувати голос коментаря.

Однак можна змінити стиль панелі голосування. Поточні варіанти — це стандартні кнопки Вгору/Вниз або використання механізму голосування у вигляді серця.

Ми використовуємо прапорець **voteStyle** наступним чином:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Ми настійно рекомендуємо робити це без коду, оскільки це також вмикає серверні перевірки. На сторінці налаштування віджету дивіться розділ "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Налаштування стилю голосування на сторінці налаштування віджету, пропонуючи стрілки вгору та вниз або голосування у вигляді серця'; title='Змінити стиль голосування' app-screenshot-end]

Голосування також можна вимкнути, дивіться `Disable Voting` вище параметрів стилю.