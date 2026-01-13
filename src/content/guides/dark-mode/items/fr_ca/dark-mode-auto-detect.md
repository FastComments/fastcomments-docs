Par défaut, FastComments détectera automatiquement si votre site a un arrière-plan sombre basé sur la « distance du noir » dans le cercle des couleurs.

Nos produits font de leur mieux à cet égard, cependant il y a presque une infinité de couleurs dans la roue des couleurs, et il peut y avoir des scénarios où l'application choisit d'utiliser le mode sombre alors que ce n'est pas approprié, et vice-versa. Cette documentation couvre comment avoir un contrôle plus précis sur cela.

#### Détails techniques

Nous détectons le mode sombre en parcourant les éléments de la page vers le haut depuis le widget de commentaires, en recherchant un arrière-plan sombre lorsque le widget se charge initialement.

Pour basculer le mode sombre après cette étape, vous devez appeler le widget pour mettre à jour sa configuration. Ceci est couvert dans la section `Configuration manuelle`.
