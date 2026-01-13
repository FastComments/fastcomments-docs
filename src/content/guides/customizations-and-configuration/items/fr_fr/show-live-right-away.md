---
[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Par défaut, les commentaires en direct sont activés. Cela signifie que si des commentaires sont ajoutés, supprimés, modifiés ou épinglés, les modifications doivent apparaître
pour tous les utilisateurs consultant le fil de commentaires en même temps.

Cependant, par défaut ces nouveaux commentaires apparaîtront sous un bouton affiché dynamiquement avec un texte similaire à "Afficher 2 nouveaux commentaires".

Si les nouveaux commentaires sont des réponses directement à la page, le bouton s'affichera en haut du fil de commentaires. S'il s'agit de réponses à un commentaire particulier, 
le bouton s'affichera sous ce commentaire.

Cela permet d'éviter que la taille de la page ne change constamment pour l'utilisateur, ce qui pourrait provoquer de la frustration lorsqu'il tente d'attraper la barre de défilement.

Pour certains cas d'utilisation, comme les enchères en direct ou les événements en ligne, ce comportement n'est pas souhaité - vous pouvez préférer que le widget de commentaires soit
plus proche d'une "boîte de chat" où les nouveaux commentaires "s'affichent immédiatement".

Donc, le nom du drapeau qui active cette fonctionnalité : **showLiveRightAway**.

Nous pouvons l'activer comme suit :

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Afficher immédiatement les commentaires en direct'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Afficher immédiatement les commentaires en direct' app-screenshot-end]

---