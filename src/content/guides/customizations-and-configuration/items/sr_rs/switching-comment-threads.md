[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Размотрили смо како је `urlId` идентификатор странице или чланка за који су коментари везани.

Такође, да резимирамо, ако није дефинисан, `urlId` ће по подразумеваној вредности бити URL текуће странице.

А шта је са SPA-овима, односно једностраничним апликацијама, где се страница или садржај за који су коментари везани мења динамички без поновног учитавања целе странице?

#### Angular, React, Vue, etc

Са нашим библиотекама као што су Angular и React, једноставно ажурирање `urlId` својства које се прослеђује видгету ће натерати видгет за коментаре да се освежи. Ово можете видети у акцији за React апликацију, на пример, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">овде</a>.

#### VanillaJS

Ако користите VanillaJS библиотеку, то је мало компликованије јер нема фрејмворка као што су Angular или React који би управљао везивањем података или пропагацијом стања.

Када иницијализујете VanillaJS видгет, он враћа неке функције које могу бити позване да га ажурирају.

Ево функционалног примера где мењамо хеш странице и ажурирамо видгет за коментаре:

[inline-code-attrs-start title = 'Пример промене хеша странице'; inline-code-attrs-end]
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

            config.url = locationString; // Ажурирамо и url, тако да обавештења могу да упућују назад на исправну страницу
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---