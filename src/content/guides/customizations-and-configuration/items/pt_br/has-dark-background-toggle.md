[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Para sites que permitem alternar o modo escuro após o carregamento inicial da página, isso é um pouco mais complexo.

Primeiro, todas as versões atuais da biblioteca do widget de Comentários (React, Vue) têm exemplos de alternância do modo escuro em seus respectivos repositórios.

Para o widget VanillaJS, precisaremos fazer um trabalho extra. Primeiro, o FastCommentsUI retorna um objeto com as funções "destroy" e "update".

Podemos simplesmente chamar a função update toda vez que quisermos atualizar a configuração do widget de comentários, como a seguir. Aqui está um exemplo completo funcionando de alternância
do modo escuro com o widget VanillaJS.

[inline-code-attrs-start title = 'Exemplo Completo de Alternância do Modo Escuro'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---