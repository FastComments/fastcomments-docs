[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Вече обяснихме как `urlId` е идентификаторът на страницата или статията, към която са свързани коментарите.

Също така, за да обобщим, ако не е дефиниран, `urlId` по подразбиране ще бъде текущият URL на страницата.

А какво за SPA, или Single-Page-Applications, при които страницата или съдържанието, към което са свързани коментарите, се променя динамично без презареждане на страницата?

#### Angular, React, Vue и др

С нашите библиотеки като Angular и React, просто обновяването на свойството `urlId`, което се подава на джаджата, ще накара джаджата за коментари да се обнови. Можете да видите това в действие за React приложението, например <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">тук</a>.

#### VanillaJS

Ако използвате библиотеката VanillaJS, това е малко по-сложно, тъй като няма рамка като Angular или React, която да обработва свързването на данни или разпространението на състоянието.

Когато инстанцирате VanillaJS джаджата, тя връща някои функции, които могат да бъдат извикани, за да я обновят.

Ето работещ пример, в който променяме хеша на страницата и обновяваме джаджата за коментари:

[inline-code-attrs-start title = 'Пример за смяна на хеша на страницата'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // We update url, too, so notifications can link back to the right page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---