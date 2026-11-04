Abaixo está o código Vanilla JS para instalar o Summary Widget. A biblioteca React também possui este widget.

[inline-code-attrs-start title = 'Instalação do Summary Widget'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

O widget encontrará automaticamente as perguntas a serem exibidas no resumo com base na configuração do widget correspondente para essa página/site.

### Filtrando Comentários por Avaliação

Quando o widget de comentários da mesma página também está na página, e ele está configurado para mostrar a resposta de cada comentarista em seu comentário, as linhas do detalhamento
se tornam clicáveis. Clicar em uma linha, por exemplo "5", filtra o widget de comentários para os comentários que deram aquela resposta. Clicar na linha novamente, ou no X no chip "Mostrando apenas"
no widget de comentários, mostra todos os comentários novamente. Isso não requer configuração: o Summary Widget encontra o widget de comentários com o mesmo `tenantId` e `urlId`.

Isso funciona para perguntas de estrelas, polegares, emoji, escala e escolha única. O detalhamento que o widget de comentários mostra acima da caixa de comentários pode ser clicado da mesma forma.

Se você precisar do widget em uma de nossas outras bibliotecas que não o possui, abra um ticket de suporte para que saibamos que devemos adicioná-lo.