Pour Page Reacts, nous devons décider de deux choses :

- Quelles images de réaction utiliser.
- Un `id` court pour nommer chaque réaction.

Optionnellement :

- Vous pouvez également définir des images séparées optionnelles pour les réactions sélectionnées/non sélectionnées.
- Vous pouvez décider si vous voulez afficher la liste des utilisateurs qui ont réagi lorsque la souris passe au-dessus de l'une des réactions. 

[inline-code-attrs-start title = 'Exemple de code pour Page Reacts'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
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

La configuration pour les bibliothèques frontend React, Angular, etc. est la même.

---