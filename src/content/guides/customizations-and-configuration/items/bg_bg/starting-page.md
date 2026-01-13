[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Когато се извличат и показват коментари, уиджът за коментари трябва да знае от коя страница да започне. По подразбиране той започва с
първата страница, рендерирайки само тази страница.

Ако желаете, точната страница, която да бъде рендерирана, може да бъде предадена на уиджета за коментари като настройка *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Обърнете внимание, че номерата на страниците започват от нула, така че горният пример рендерира втората страница.