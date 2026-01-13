Voor Pagina-reacties moeten we twee dingen beslissen:

- Welke afbeeldingen voor reacties te gebruiken.
- Een korte `id` om elke reactie te benoemen.

Optioneel:

- Je kunt ook optioneel afzonderlijke afbeeldingen definiëren voor geselecteerde/niet-geselecteerde reacties.
- Je kunt bepalen of je de lijst met gebruikers die hebben gereageerd wilt tonen wanneer je met de muis over een van de reacties beweegt. 

[inline-code-attrs-start title = 'Voorbeeldcode Pagina-reacties'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="page-reacts-example"></div>
<script>
    window.FastCommentsUI(document.getElementById('page-reacts-example'), {
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
    });
</script>
[inline-code-end]

De configuratie voor de React-, Angular- en andere frontend-bibliotheken is hetzelfde.

---