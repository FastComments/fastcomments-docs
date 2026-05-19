#### Comment les commentaires apparaissent dans vos cours

Une fois l’intégration LTI activée et l’application externe installée, FastComments fonctionne automatiquement en fonction des emplacements que vous avez configurés :

#### Vue des devoirs

Si l’emplacement **Vue des devoirs** est activé, les commentaires apparaissent automatiquement sous chaque devoir du cours. Les étudiants et les enseignants voient une section de commentaires en fil lorsqu’ils consultent un devoir — aucune configuration supplémentaire par devoir n’est nécessaire.

Chaque devoir dispose de son propre fil de commentaires.

#### Bouton de l’éditeur de contenu riche

Si l’emplacement **Bouton de l’éditeur** est activé, les enseignants peuvent intégrer FastComments dans tout contenu utilisant l’éditeur de contenu riche :

1. Modifiez une **Page**, un **Quiz**, ou une **Annonce**.
2. Dans la barre d’outils de l’éditeur de contenu riche, cliquez sur le bouton **FastComments**.
3. FastComments est automatiquement intégré dans le contenu.
4. Enregistrez la page.

Lorsque les étudiants consultent la page, le widget FastComments intégré se charge avec un fil de commentaires unique pour cette page.

#### SSO automatique

Dans les deux emplacements, les étudiants sont connectés automatiquement via leur compte Canvas. Les noms, adresses e-mail et avatars sont synchronisés via le lancement LTI, aucune connexion séparée n’est nécessaire.

#### Verrouiller l’accès public (recommandé)

Par défaut, les données de commentaire FastComments sont lisibles publiquement. Toute personne capable de deviner l’URL d’un fil ou un point de terminaison API peut voir ses commentaires, même en dehors de Canvas. Pour les discussions de cours, vous souhaiterez très probablement restreindre la visualisation aux étudiants inscrits uniquement.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Exiger la SSO pour afficher les commentaires** activée, puis définissez le niveau de sécurité sur **SSO sécurisé** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Voir [Protéger les fils de commentaires avec l’authentification unique](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou une seule page.