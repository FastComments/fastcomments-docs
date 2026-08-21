Il existe deux façons d’interdire aux utilisateurs de commenter sur votre site avec FastComments.

La première consiste, si vous connaissez déjà leur adresse e‑mail, à la saisir sur la page <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utilisateurs bannis</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Liste des utilisateurs bannis sous Modération des commentaires, avec les adresses e‑mail bannies et un bouton pour ajouter un nouveau bannissement'; title='Page des utilisateurs bannis' app-screenshot-end]

Cette page est accessible via Modération des commentaires -> Utilisateurs bannis

Lorsque nous voulons bannir un utilisateur, nous pouvons choisir un type, soit Permanent, soit Bannissement permanent en mode fantôme :

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Formulaire de nouveau bannissement avec un champ e‑mail et un choix de type de bannissement : Permanent ou Bannissement permanent en mode fantôme'; title='Bannir un utilisateur' app-screenshot-end]

La deuxième façon de bannir un utilisateur consiste à cliquer sur le bouton de bannissement placé sur chaque commentaire de la page de modération des commentaires.

Lorsque nous cliquons sur le bouton de bannissement, des options vous sont présentées, où nous pouvons spécifier le type de bannissement et sa durée.

### Alias d'e‑mail

Lors du bannissement d’un utilisateur par e‑mail, FastComments ignore automatiquement les alias `+`. Par exemple, bannir `user+alias@gmail.com` bannira également `user@gmail.com` ainsi que toute autre variante `+` de cette adresse, comme `user+other@gmail.com`.

### Bannissements fantômes

Un bannissement fantôme est un type de bannissement qui donne l’impression que le commentaire ou le vote de l’utilisateur a été enregistré avec succès, alors qu’en réalité ce n’est pas le cas. Cela peut être souhaitable dans certaines situations.

### Bannissement par adresse IP

À moins qu’un locataire ne souhaite se désinscrire, FastComments prend en charge le bannissement par IP en stockant une version hachée de l’adresse IP du commentateur.

### Recherche d’utilisateurs bannis

Lorsque votre liste dépasse une ou deux pages, vous pouvez la restreindre à l’aide de la ligne de recherche située au-dessus du tableau.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Ligne de recherche sur la page des utilisateurs bannis avec un menu déroulant Rechercher par, un menu déroulant Correspondance et un champ Valeur'; title='Recherche d\'utilisateurs bannis' app-screenshot-end]

Il y a trois contrôles :

- **Search By** (Rechercher par) choisit le champ dans lequel chercher : Tout champ, E‑mail, Nom, Banni par, ou Banni pour dire. Les quatre derniers correspondent aux colonnes du même nom dans le tableau.  
- **Match** (Correspondance) détermine la façon de comparer. **Contains** (Contient) trouve votre valeur n’importe où dans le champ, et **Equals** (Égal) correspond à l’intégralité du champ.  
- **Value** (Valeur) est le texte à rechercher.

Chaque champ est comparé sans tenir compte de la casse, ainsi rechercher `SPAMMER@EXAMPLE.COM` trouve un bannissement stocké sous `spammer@example.com`.

Quelques points utiles à connaître :

- **Banned For Saying** (Banni pour dire) recherche le texte du commentaire qui a conduit au bannissement de l’utilisateur. C’est ainsi que vous trouvez tous les utilisateurs bannis pour une phrase particulière.  
- **Banned By** (Banni par) recherche le nom du modérateur qui a émis le bannissement, ce qui est utile pour examiner les décisions d’un autre modérateur.  
- Les bannissements génériques sont stockés avec leur `*`, ainsi une recherche **Contains** pour `bademail.com` trouve un bannissement `*@bademail.com`.  
- **Name** (Nom) correspond au nom affiché dans la colonne Nom, il trouve donc un utilisateur même s’il a changé de nom depuis le bannissement, et même si vous avez créé le bannissement en saisissant une adresse e‑mail et qu’aucun nom n’a été enregistré à ce moment. Le nom enregistré sur le bannissement correspond également, ainsi rechercher soit l’ancien, soit le nom actuel fonctionne.  
- **Any Field** (Tout champ) recherche simultanément l’e‑mail, le nom, le modérateur qui a banni et le texte du commentaire banni.

Votre recherche fait partie de l’URL de la page, vous pouvez donc partager une liste filtrée avec d’autres modérateurs de la même manière que vous partagez d’autres liens de modération. La pagination des résultats conserve la recherche appliquée, démarrer une nouvelle recherche vous ramène à la première page, et **Clear** (Effacer) revient à la liste complète.