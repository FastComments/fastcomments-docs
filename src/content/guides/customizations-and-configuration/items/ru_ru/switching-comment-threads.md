[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Мы уже объясняли, что `urlId` — это идентификатор страницы или статьи, к которой привязаны комментарии.

Также, напомним, если `urlId` не задан, он по умолчанию будет равен текущему URL страницы.

А как быть с SPA, или Single-Page-Applications, где страница или содержимое, к которому привязаны комментарии, меняется
динамически без полной перезагрузки страницы?

#### Angular, React, Vue и т. д.

В наших библиотеках, таких как Angular и React, простое обновление свойства `urlId`, передаваемого виджету,
вызывает обновление виджета комментариев. Например, вы можете увидеть это в действии для React-приложения <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">здесь</a>.

#### VanillaJS

Если вы используете библиотеку VanillaJS, это немного сложнее, так как нет фреймворка вроде Angular или React,
который бы обрабатывал привязку данных или распространение состояния.

Когда вы создаёте экземпляр виджета VanillaJS, он возвращает несколько функций, которые можно вызвать для его обновления.

Вот рабочий пример, в котором мы меняем хэш страницы и обновляем виджет комментариев:

[inline-code-attrs-start title = 'Пример переключения хэша страницы'; inline-code-attrs-end]
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