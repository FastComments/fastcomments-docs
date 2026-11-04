[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Објаснили смо како је `urlId` идентификатор странице или чланка на који су коментари везани.

Такође, да поновимо, ако није дефинисан, `urlId` ће подразумевано бити URL тренутне странице.

А шта је са SPA апликацијама, односно Single-Page-Application апликацијама, где се страница или садржај на који су коментари везани мења
динамички, без поновног учитавања странице?

#### Angular, React, Vue, итд

Са нашим библиотекама као што су Angular и React, једноставна измена `urlId` својства које се прослеђује виџету
изазваће освежавање виџета за коментаре. Ово можете видети у акцији за React апликацију, на пример, <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">овде</a>.

#### VanillaJS

Ако користите VanillaJS библиотеку, нешто је компликованије јер не постоји фрејмворк попут Angular или React
који би обрадио data binding или пропагацију стања.

Када инстанцирате VanillaJS виџет, он враћа неке функције које се могу позвати да би се ажурирао.

Ево функционалног примера у коме мењамо hash странице и ажурирамо виџет за коментаре:

[inline-code-attrs-start title = 'Пример промене hash вредности странице'; inline-code-attrs-end]
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

            config.url = locationString; // Ажурирамо и url, како обавештења могу да воде ка правој страници
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]