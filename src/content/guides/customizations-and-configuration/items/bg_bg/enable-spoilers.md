[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Можем да активираме поддръжката на спойлери, като зададем флага **enableSpoilers** на true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Включване на спойлери'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на уиджета, вижте опцията „Enable Spoilers“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Страница за персонализиране на уиджет с отметка „Enable Spoilers“ за добавяне на бутона SPOILER в редактора'; title='Включване на спойлери' app-screenshot-end]

Когато текстът е маркиран и сега видимият бутон `SPOILER` се кликне, текстът ще бъде скрит, докато потребителят не задържи мишката върху него. За тъмен режим правим същото, но с различни
цветове, които по‑добре съвпадат с тъмния режим.

Това също е съвместимо с WYSIWYG редактора.