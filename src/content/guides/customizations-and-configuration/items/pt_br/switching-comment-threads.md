[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Cobrimos como `urlId` é o id da página ou do artigo ao qual os comentários estão vinculados.

Além disso, para recapitular, se não definido o `urlId` será padrão para a URL da página atual.

O que acontece com SPAs, ou Single-Page-Applications, onde a página ou o conteúdo ao qual os comentários estão vinculados muda dinamicamente sem um recarregamento completo da página?

#### Angular, React, Vue, etc

Com nossas bibliotecas, como Angular e React, simplesmente atualizar a propriedade `urlId` passada para o widget fará com que o widget de comentários seja atualizado. Você pode ver isso em ação para o app React, por exemplo, <a href="https://github.com/FastComments/fastcomments-react/blob/master/example/src/PaginatedApp.tsx#L39" target="_blank">aqui</a>.

#### VanillaJS

Se você estiver usando a biblioteca VanillaJS é um pouco mais complicado, já que não existe um framework como Angular ou React para lidar com data binding ou propagação de estado.

Quando você instancia o widget VanillaJS, ele retorna algumas funções que podem ser chamadas para atualizá-lo.

Aqui está um exemplo funcional onde mudamos o hash da página e atualizamos o widget de comentários:

[inline-code-attrs-start title = 'Exemplo de troca do hash da página'; inline-code-attrs-end]
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

            config.url = locationString; // Também atualizamos a url, para que as notificações possam vincular de volta à página correta
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]

---