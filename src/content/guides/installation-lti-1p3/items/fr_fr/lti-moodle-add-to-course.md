Ce guide couvre l'ajout de FastComments à un cours Moodle 4.x après qu'un administrateur du site a enregistré l'outil et configuré son affichage dans le sélecteur d'activités. Si FastComments n'est pas encore enregistré, consultez d'abord le guide d'enregistrement Moodle.

#### Ouvrir le cours en mode édition

1. Connectez-vous à Moodle en tant qu'enseignant éditeur (ou rôle supérieur) pour le cours.
2. Ouvrez le cours.
3. Activez le **Mode édition** à l'aide de l'interrupteur en haut à droite de l'en-tête du cours.

Moodle 4.x a remplacé le menu déroulant « Ajouter une activité ou une ressource » utilisé par la version 3.x par une boîte de dialogue plein écran de sélection d'activités. Moodle 4.5 conserve le même sélecteur mais ajoute une ligne d'éléments favoris (étoilés) en haut, ce qui permet d'épingler FastComments une seule fois pour y accéder plus rapidement dans les sections suivantes.

#### Ajouter l'activité FastComments

1. Faites défiler jusqu'à la section du cours (thème ou semaine) où la discussion doit se trouver.
2. Cliquez sur **Ajouter une activité ou une ressource** en bas de cette section.
3. Dans la boîte de dialogue du sélecteur, sélectionnez **FastComments**. Si vous ne le voyez pas, passez à la section des pièges ci-dessous.

Le formulaire de paramètres de l'activité s'ouvre. Les champs importants :

- **Nom de l'activité** (obligatoire). Affiché sur la page du cours et dans le carnet de notes. Exemple : `Week 3 Discussion`.
- **Description de l'activité**. Texte d'introduction facultatif affiché au-dessus du fil de commentaires.
- **Afficher la description sur la page du cours**. Cochez cette case si vous souhaitez que la description soit visible sans avoir à cliquer dans l'activité.
- **Outil préconfiguré**. Réglé sur `FastComments` (sélectionné automatiquement lors du lancement depuis le sélecteur). Ne pas modifier.
- **Conteneur de lancement**. Réglez sur **Nouvelle fenêtre**. Consultez la section des pièges pour comprendre pourquoi « Même fenêtre » pose problème dans certains déploiements Moodle.
- **URL de l'outil**, **Clé publique**, **Secret partagé**, **Paramètres personnalisés**. Laissez vides. L'enregistrement dynamique a géré ces éléments au niveau du site.

Faites défiler vers le bas et cliquez sur **Enregistrer et revenir au cours** (ou **Enregistrer et afficher** pour ouvrir l'activité immédiatement).

L'activité apparaît sous forme de ligne dans la section avec l'icône FastComments. Les étudiants cliquent sur la ligne pour ouvrir le fil de commentaires.

#### Intégrer FastComments en ligne avec l'éditeur

Pour un fil de discussion à l'intérieur d'une Page, d'un chapitre de Livre, d'une Leçon ou de toute autre ressource utilisant l'éditeur Atto ou TinyMCE :

1. Ouvrez la ressource en mode édition.
2. Placez le curseur à l'endroit où le fil doit apparaître.
3. Dans la barre d'outils de l'éditeur, cliquez sur le bouton **LTI** / **Outil externe**. Dans Atto, il est intitulé « Insérer du contenu LTI Advantage ». Dans TinyMCE (par défaut dans Moodle 4.3+), il se trouve dans le menu **Plus** sous **Outils externes**.
4. Choisissez **FastComments** dans la liste des outils.
5. FastComments ouvre un sélecteur de deep-linking. Confirmez le titre du fil et cliquez sur **Intégrer**.
6. L'éditeur insère un bloc d'espace réservé LTI. Enregistrez la ressource.

Chaque instance intégrée est un fil distinct identifié par l'ID de l'élément de contenu du deep-link, ainsi une Page avec trois intégrations FastComments obtient trois fils indépendants.

#### Restreindre l'accès et paramètres de groupe

Les paramètres d'activité Moodle standard s'appliquent aux activités FastComments :

- **Paramètres communs de module** > **Mode de groupe**. Régler ce paramètre sur **Groupes séparés** ou **Groupes visibles** ne divise pas FastComments en fils par groupe à lui seul. Le mode de groupe de Moodle ne filtre que le carnet de notes et la liste des membres. Pour exécuter un fil séparé par groupe, ajoutez une activité FastComments par groupe et utilisez **Restreindre l'accès** pour délimiter chacune.
- **Restreindre l'accès** > **Ajouter une restriction**. Prend en charge les conditions Moodle standard : **Date**, **Note**, **Groupe**, **Groupement**, **Profil utilisateur**, ainsi que des ensembles de restrictions imbriqués. Utilisez **Groupe** pour verrouiller une activité FastComments à un seul groupe.
- **Achèvement d'activité**. Réglez sur **Les étudiants doivent consulter cette activité pour la terminer** si vous souhaitez le suivi d'achèvement. FastComments ne renvoie actuellement pas d'événement d'achèvement à Moodle au-delà du lancement.

#### Correspondance des rôles

FastComments lit la revendication LTI `roles` envoyée par Moodle à chaque lancement et l'associe comme suit :

- Moodle **Gestionnaire** ou **Administrateur du site** -> FastComments **admin**
- Moodle **Enseignant éditeur** ou **Enseignant non éditeur** -> FastComments **moderator**
- Moodle **Étudiant** -> FastComments **commenter**
- Moodle **Invité** -> lecture seule

Les administrateurs peuvent supprimer n'importe quel commentaire, bannir des utilisateurs et modifier les paramètres des fils. Les modérateurs peuvent supprimer et approuver les commentaires dans le fil dans lequel ils ont été lancés. Les rôles Moodle personnalisés héritent de la correspondance de l'archétype dont ils proviennent.

#### Ce que voient les étudiants

Les étudiants cliquent sur l'activité FastComments (ou font défiler jusqu'au bloc intégré dans une Page ou un Livre). Moodle envoie leur identité à FastComments via le lancement LTI :

- Pas d'écran de connexion. FastComments les connecte à l'aide du compte Moodle.
- Leur nom affiché, leur e-mail et leur avatar proviennent de Moodle.
- Le fil est délimité par `(site Moodle, cours, ID du lien de ressource)`, ainsi la même activité dupliquée dans un autre cours obtient un nouveau fil.
- Les réponses en fil d'discussion, les votes et les notifications fonctionnent de la même manière qu'un fil FastComments autonome.

#### Verrouiller l'accès public (recommandé)

Par défaut, les données de commentaires FastComments sont lisibles publiquement. Quiconque peut deviner l'URL d'un fil ou un point d'accès API peut consulter ses commentaires, même en dehors de Moodle. Pour des discussions de cours, vous voudrez presque certainement restreindre la consultation aux seuls étudiants inscrits.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Require SSO To View Comments** activé, puis définissez le niveau de sécurité sur **Secure SSO** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Consultez [Protéger les fils de commentaires avec l'authentification unique](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment délimiter la règle à un seul domaine ou à une seule page.

#### Pièges Moodle

**FastComments absent du sélecteur d'activités.** L'administrateur du site a enregistré l'outil mais n'a pas réglé **Utilisation de la configuration de l'outil** sur **Afficher dans le sélecteur d'activités et comme outil préconfiguré**. Corrigez cela sous **Administration du site** > **Plugins** > **Modules d'activité** > **Outil externe** > **Gérer les outils** > icône d'engrenage sur la tuile FastComments.

**Le lancement échoue ou affiche un cadre vide lorsque réglé sur « Même fenêtre ».** Les cookies de session de Moodle utilisent `SameSite=Lax` par défaut, et certains navigateurs les suppriment lors de la requête POST inter-sites que LTI 1.3 utilise pour revenir depuis FastComments. Réglez le **Conteneur de lancement** sur **Nouvelle fenêtre** pour l'activité. C'est une exigence impérative pour FastComments intégré dans une Page ou un Livre, car le chemin de lancement intégré à l'éditeur ouvre toujours une nouvelle fenêtre.

**La revendication `iss` est l'URL du site Moodle, pas un ID de tenant.** FastComments utilise l'URL du site Moodle (la valeur de configuration `wwwroot`) comme émetteur LTI. Si votre instance Moodle déménage vers un nouveau domaine ou si vous modifiez `wwwroot`, les fils FastComments existants restent liés à l'ancien émetteur et ne correspondront plus aux nouveaux lancements. Réenregistrez l'outil avec la nouvelle URL et migrez les fils via l'administration FastComments si nécessaire.

**Sauvegarde et restauration d'activité.** Sauvegarder un cours et le restaurer dans un nouveau cours crée de nouveaux ID de liens de ressources, ainsi les activités FastComments restaurées commencent avec des fils vides. Le cours d'origine conserve les fils d'origine. C'est un comportement voulu, pas un bug.

**TinyMCE par défaut dans Moodle 4.5.** Moodle 4.5 est livré avec TinyMCE comme éditeur par défaut pour les nouvelles installations. Le bouton Outil externe se trouve dans le menu **Plus** (`...`) plutôt que dans la barre d'outils principale. Les anciens sites mis à niveau depuis la version 4.1 conservent Atto, sauf si un administrateur a modifié le paramètre par défaut.