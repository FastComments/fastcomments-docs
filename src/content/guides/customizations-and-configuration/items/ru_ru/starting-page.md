[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

При получении и отображении комментариев виджету комментариев нужно знать, с какой страницы начинать. По умолчанию он начинает с
первой страницы и отображает только эту страницу.

Если нужно, точную страницу для отображения можно передать виджету комментариев через настройку *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Обратите внимание, что нумерация страниц начинается с нуля, поэтому приведённый выше пример отображает вторую страницу.