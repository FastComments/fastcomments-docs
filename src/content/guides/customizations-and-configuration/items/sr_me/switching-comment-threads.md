[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Покрили смо како је `urlId` ид странице или чланка за које су коментари везани.

Такође, да поновимо, ако није дефинисан, `urlId` ће по дифолту бити тренутни URL странице.

Шта је са SPAs, или Single-Page-Applications, где се страница или садржај на који су коментари везани мења
динамички без поновног учитавања странице?

#### Angular, React, Vue, итд

Са нашим библиотекама као што су Angular и React, само ажурирање својства `urlId` које се прослеђује видгету
ће изазвати освежавање видгета за коментаре. Можете то видјети у пракси за React апликацију, на примјер, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">овде</a>.

#### VanillaJS

Ако користите VanillaJS библиотеку, то је мало компликованије јер не постоји фрејмворк као Angular или React
који би руковао везивањем података или пропагацијом стања.

Када инстанцирате VanillaJS видгет, он враћа неке функције које се могу позвати да би се ажурирао.

Ево функционалног примјера у којем мијењамо хеш странице и ажурирамо видгет за коментаре:

[inline-code-attrs-start title = 'Пример промјене хеша странице'; inline-code-attrs-end]
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

            config.url = locationString; // Ажурирамо и url, тако да обавештења могу упутити назад на праву страницу
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---