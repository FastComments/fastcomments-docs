[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

По подразбиране FastComments уиджетът ще се преоразмерява вертикално, за да побере всички видими коментари. Пагинацията се реализира чрез бутон "Прегледай следващите"
в края на текущата страница, тъй като установихме, че това е взаимодействието, което се усеща най-удобно за повечето потребители.

Въпреки това има случаи, в които е предпочитано безкрайно превъртане. Например, ние използваме тази функция в нашия продукт Stream Chat.

Можем да скрием бутоните "Прегледай следващите" и да превключим към безкрайно превъртане, като зададем флага **enableInfiniteScrolling** на true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Това също изисква добавяне на персонализиран CSS. Добавете персонализиран CSS за селектора `.comments`, за да позволите превъртане, например:

[inline-code-attrs-start title = 'Активиране на безкрайно превъртане'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Пълният работещ пример би бил:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

В горния пример използваме свойството `customCSS`, но се препоръчва вместо това да се използва потребителският интерфейс за конфигурация на уиджета по причини за производителност. [Вижте документацията за персонализиран CSS.](/guide-customizations-and-configuration.html#custom-css)