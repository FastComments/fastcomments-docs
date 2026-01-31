Voor Page Reacts moeten we twee dingen bepalen:

- Welke afbeeldingen voor reacties te gebruiken.
- Een korte `id` om elke reactie te benoemen.

Optioneel:

- U kunt ook optioneel afzonderlijke afbeeldingen definiëren voor geselecteerde/niet-geselecteerde reacties.
- U kunt kiezen of u de lijst met gebruikers die hebben gereageerd wilt tonen wanneer u met de muis over een van de reacties gaat. 

[inline-code-attrs-start title = 'Pagina-reacties - Voorbeeldcode'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.fcConfigs = [{
        target: '#page-reacts-example',
        tenantId: 'demo',
        pageReactConfig: {
            showUsers: true,
            reacts: [
                {id: 'droll', src: 'https://docs.fastcomments.com/images/emojis/droll.png'},
                {id: 'he', src: 'https://docs.fastcomments.com/images/emojis/heart-eyes.png'},
                {id: 'laugh', src: 'https://docs.fastcomments.com/images/emojis/laugh.png'},
                {id: 'puke', src: 'https://docs.fastcomments.com/images/emojis/puke.png', selectedSrc: 'https://docs.fastcomments.com/images/emojis/puke-bw.png' },
                {id: 'rofl', src: 'https://docs.fastcomments.com/images/emojis/rofl.png' },
            ]
        }
    }];
</script>
[inline-code-end]

De configuratie voor de frontendbibliotheken zoals React, Angular en dergelijke is hetzelfde.