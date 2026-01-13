[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Можем да активираме поддръжката на спойлери, като зададем флага **enableSpoilers** на true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Това може да се направи и без код. На страницата за персонализиране на джаджата, вижте опцията "Активиране на спойлери".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Когато текстът е маркиран, и се кликне върху вече видимия бутон `SPOILER`, текстът ще бъде маскиран, докато потребителят не премести курсора на мишката върху него. За тъмен режим правим същото, с различни
цветове, които по-добре съответстват на тъмния режим.

Това е съвместимо и с WYSIWYG редактора.