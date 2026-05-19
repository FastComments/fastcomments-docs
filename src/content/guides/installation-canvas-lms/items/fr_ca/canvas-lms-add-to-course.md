#### Comment les commentaires apparaissent dans vos cours

Une fois l'intégration LTI activée et que l'application externe est installée, FastComments fonctionne automatiquement selon les emplacements que vous avez configurés :

#### Affichage du devoir

Si l'emplacement **Affichage du devoir** est activé, les commentaires apparaissent automatiquement sous chaque devoir du cours. Les étudiants et les enseignants voient une section de commentaires en fils lorsqu'ils consultent un devoir — aucune configuration supplémentaire n'est requise par devoir.

Chaque devoir a son propre fil de commentaires distinct.

#### Bouton de l'éditeur de contenu enrichi

Si l'emplacement **Bouton de l'éditeur** est activé, les enseignants peuvent intégrer FastComments dans tout contenu utilisant l'Éditeur de contenu enrichi :

1. Modifiez une **Page**, un **Quiz**, ou une **Annonce**.
2. Dans la barre d'outils de l'Éditeur de contenu enrichi, cliquez sur le bouton **FastComments**.
3. FastComments est automatiquement intégré au contenu.
4. Enregistrez la page.

Lorsque les étudiants consultent la page, le widget FastComments intégré se charge avec un fil de commentaires unique à cette page.

#### SSO automatique

Dans les deux emplacements, les étudiants sont automatiquement connectés via leur compte Canvas. Les noms, courriels et avatars sont synchronisés lors du lancement LTI, aucun autre identifiant n'est nécessaire.

#### Verrouiller l'accès public (recommandé)

Par défaut, les données de commentaires FastComments sont lisibles publiquement. Toute personne pouvant deviner l'URL d'un fil ou un endpoint API peut voir ses commentaires, même à l'extérieur de Canvas. Pour les discussions de cours, vous voudrez presque certainement restreindre l'affichage aux étudiants inscrits seulement.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Require SSO To View Comments** activée, puis définissez le niveau de sécurité sur **Secure SSO** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Voir [Protéger les fils de commentaires avec l'authentification unique](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou une seule page.