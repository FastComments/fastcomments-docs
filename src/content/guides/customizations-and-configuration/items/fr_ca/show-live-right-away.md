[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Par défaut, la fonctionnalité de commentaires en direct est activée. Cela signifie que si des commentaires sont ajoutés, supprimés, modifiés, ou épinglés, les changements doivent apparaître
pour tous les utilisateurs qui consultent le fil de commentaires en même temps.

Cependant, par défaut ces nouveaux commentaires apparaîtront sous un bouton affiché dynamiquement avec un texte similaire à "Afficher 2 nouveaux commentaires".

Si les nouveaux commentaires sont des réponses directement à la page, le bouton s'affichera en haut du fil de commentaires. S'ils sont des réponses à un commentaire particulier, 
le bouton s'affichera sous ce commentaire.

Ceci permet d'empêcher la taille de la page de changer constamment pour l'utilisateur, ce qui pourrait causer de la frustration lorsqu'on tente d'attraper la barre de défilement.

Pour certains cas d'utilisation, comme les enchères en direct ou les événements en ligne, ce comportement n'est pas souhaité — vous pouvez vouloir que le widget de commentaires soit
plus comme une boîte de « chat » où les nouveaux commentaires « s'affichent immédiatement ».

D'où le nom du flag qui active cette fonctionnalité : **showLiveRightAway**.

Nous pouvons l'activer comme suit :

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]