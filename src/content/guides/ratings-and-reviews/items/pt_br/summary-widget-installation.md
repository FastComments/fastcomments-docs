---
Abaixo está o código Vanilla JS para instalar o Summary Widget. A biblioteca React também possui este widget.

[inline-code-attrs-start title = 'Instalação do Widget de Resumo'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

O widget encontrará automaticamente as perguntas a serem exibidas no resumo com base na configuração correspondente do widget para essa página/site.

Se você precisar do widget em uma de nossas outras bibliotecas que não o possui, abra um ticket de suporte para que saibamos adicioná-lo.

---