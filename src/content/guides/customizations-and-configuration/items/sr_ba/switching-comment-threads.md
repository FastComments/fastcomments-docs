[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Објаснили смо како је `urlId` идентификатор странице или чланка на који су коментари везани.

Такође, у кратком прегледу, ако није дефинисан, `urlId` ће подразумјевано бити URL тренутне странице.

А шта је са SPA-овима, или апликацијама једне странице, гдје се страница или садржај на који су коментари везани мијења
динамички без поновног учитавања странице?

#### Angular, React, Vue, etc

Са нашим библиотекама као што су Angular и React, једноставно ажурирање својства `urlId` које се прослијеђује видгету
ће узроковати освежавање коментар-видгета. Ово можете видјети у пракси за React апликацију, на примјер, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">овдје</a>.

#### VanillaJS

Ако користите VanillaJS библиотеку, ствари су мало сложеније јер нема фрејмворка попут Angular-а или React-а
који би руковао везивањем података или пропагацијом стања.

Када инстанцирате VanillaJS видгет, он враћа неке функције које се могу позвати да га ажурирају.

Ево функционалног примјера у коме мијењамо хеш странице и ажурирамо коментар-видгет:

[inline-code-attrs-start title = 'Примјер промјене хеша странице'; inline-code-attrs-end]
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