[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Par défaut, les commentaires en direct sont activés. Cela signifie que si des commentaires sont ajoutés, supprimés, modifiés ou épinglés, les changements doivent apparaître pour tous les utilisateurs qui consultent le fil de commentaires en même temps.

Cependant, par défaut, ces nouveaux commentaires apparaîtront sous un bouton affiché dynamiquement avec un texte similaire à « Show 2 New Comments ».

Si les nouveaux commentaires sont des réponses directement à la page, le bouton s'affichera en haut du fil de commentaires. S'ils sont des réponses à un commentaire particulier, le bouton s'affichera sous ce commentaire.

Cela permet d'éviter que la taille de la page change constamment pour l'utilisateur, ce qui pourrait provoquer de la frustration lorsqu'il essaie de saisir la barre de défilement.

Pour certains cas d'utilisation, comme les enchères en direct ou les événements en ligne, ce comportement n'est pas souhaité – vous pouvez vouloir que le widget de commentaires se comporte davantage comme une boîte de « chat » où les nouveaux commentaires « s'affichent immédiatement ».

Ainsi, le nom du drapeau qui active cette fonctionnalité : **showLiveRightAway**.

Nous pouvons l'activer comme suit :

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Afficher les commentaires en direct immédiatement'; code-example-end]

Cela peut être personnalisé sans code, sur la page de personnalisation du widget :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Paramètre de réduction des commentaires en direct activé afin que les nouveaux commentaires apparaissent instantanément au lieu d\'être derrière un bouton'; title='Afficher les commentaires en direct immédiatement' app-screenshot-end]