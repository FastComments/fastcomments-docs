Ce guide couvre l’ajout de FastComments à un cours Moodle 4.x après qu’un administrateur du site ait enregistré l’outil et l’ait configuré pour qu’il apparaisse dans le sélecteur d’activités. Si FastComments n’est pas encore enregistré, consultez d’abord le guide d’enregistrement Moodle.

#### Open the Course in Edit Mode

1. Connectez-vous à Moodle en tant qu’enseignant avec droit d’édition (ou rôle supérieur) pour le cours.
2. Ouvrez le cours.
3. Activez le **mode édition** en utilisant l’interrupteur dans le coin supérieur droit de l’en-tête du cours.

Moodle 4.x a remplacé le menu déroulant hérité « Add an activity or resource » utilisé en 3.x par une boîte de sélection d’activités en plein écran. Moodle 4.5 conserve le même sélecteur mais ajoute une rangée d’étoiles/favoris en haut, donc épingler FastComments une fois permet d’y accéder plus rapidement dans les sections suivantes.

#### Add the FastComments Activity

1. Faites défiler jusqu’à la section du cours (thème ou semaine) où la discussion doit se trouver.
2. Cliquez sur **Add an activity or resource** au bas de cette section.
3. Dans la boîte de sélection, choisissez **FastComments**. Si vous ne le voyez pas, passez à la section des points d’attention ci-dessous.

Le formulaire de paramètres de l’activité s’ouvre. Les champs importants :

- **Activity name** (obligatoire). Affiché sur la page du cours et dans le carnet de notes. Exemple : `Week 3 Discussion`.
- **Activity description**. Texte d’introduction optionnel affiché au-dessus du fil de commentaires.
- **Show description on course page**. Cochez ceci si vous voulez que la description soit visible sans ouvrir l’activité.
- **Preconfigured tool**. Réglé sur `FastComments` (sélection automatique lors du lancement depuis le sélecteur). Ne pas modifier.
- **Launch container**. Réglez sur **New window**. Voir la section des points d’attention pour comprendre pourquoi « Same window » ne fonctionne pas dans certaines installations Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Laissez vides. L’enregistrement dynamique s’occupe de ces éléments au niveau du site.

Descendez en bas et cliquez sur **Save and return to course** (ou **Save and display** pour ouvrir l’activité immédiatement).

L’activité apparaît comme une ligne dans la section avec l’icône FastComments. Les étudiants cliquent sur la ligne pour ouvrir le fil de commentaires.

#### Embed FastComments Inline with the Editor

Pour insérer un fil à l’intérieur d’une Page, d’un chapitre de Book, d’une Lesson ou de toute autre ressource qui utilise l’éditeur Atto ou TinyMCE :

1. Ouvrez la ressource en mode édition.
2. Placez le curseur à l’endroit où le fil doit apparaître.
3. Dans la barre d’outils de l’éditeur, cliquez sur le bouton **LTI** / **External tool**. Dans Atto, il est intitulé « Insert LTI Advantage content ». Dans TinyMCE (par défaut dans Moodle 4.3+), il se trouve sous le menu **More** en tant que **External tools**.
4. Choisissez **FastComments** dans la liste des outils.
5. FastComments ouvre un sélecteur de deep-linking. Confirmez le titre du fil et cliquez sur **Embed**.
6. L’éditeur insère un bloc d’espace réservé LTI. Enregistrez la ressource.

Chaque instance intégrée est un fil distinct identifié par l’ID de l’élément de contenu deep-link, donc une Page contenant trois intégrations FastComments aura trois fils indépendants.

#### Restrict Access and Group Settings

Les paramètres d’activité standard de Moodle s’appliquent aux activités FastComments :

- **Common module settings** > **Group mode**. Le fait de régler ceci sur **Separate groups** ou **Visible groups** ne divise pas automatiquement FastComments en fils par groupe. Le mode de groupe de Moodle ne fait que filtrer le carnet de notes et la liste des membres. Pour exécuter un fil distinct par groupe, ajoutez une activité FastComments par groupe et utilisez **Restrict access** pour limiter chacune.
- **Restrict access** > **Add restriction**. Prend en charge les conditions standard de Moodle : **Date**, **Grade**, **Group**, **Grouping**, **User profile**, et des ensembles de restrictions imbriqués. Utilisez **Group** pour verrouiller une activité FastComments à un seul groupe.
- **Activity completion**. Réglez sur **Students must view this activity to complete it** si vous voulez le suivi d’achèvement. FastComments ne rapporte pas actuellement d’événement d’achèvement à Moodle au-delà du lancement.

#### Role Mapping

FastComments lit la réclamation LTI `roles` que Moodle envoie à chaque lancement et la mappe comme suit :

- Moodle **Manager** ou **Site administrator** -> FastComments **admin**
- Moodle **Editing teacher** ou **Non-editing teacher** -> FastComments **moderator**
- Moodle **Student** -> FastComments **commenter**
- Moodle **Guest** -> lecture seule

Les administrateurs peuvent supprimer n’importe quel commentaire, bannir des utilisateurs et modifier les paramètres du fil. Les modérateurs peuvent supprimer et approuver des commentaires dans le fil dans lequel ils ont lancé l’activité. Les rôles Moodle personnalisés héritent du mapping de l’archétype à partir duquel ils ont été clonés.

#### What Students See

Les étudiants cliquent sur l’activité FastComments (ou font défiler jusqu’au bloc intégré dans une Page ou un Book). Moodle envoie leur identité à FastComments via le lancement LTI :

- Aucun écran de connexion. FastComments les connecte en utilisant le compte Moodle.
- Leur nom d’affichage, courriel et avatar proviennent de Moodle.
- Le fil est limité à « (site Moodle, cours, resource link ID) », donc la même activité dupliquée dans un autre cours reçoit un fil neuf.
- Les réponses en arbre, le vote et les notifications fonctionnent de la même façon qu’un fil FastComments autonome.

#### Lock Down Public Access (Recommended)

Par défaut, les données de commentaires FastComments sont lisibles publiquement. Toute personne capable de deviner l’URL du fil ou le point de terminaison API peut voir ses commentaires, même en dehors de Moodle. Pour les discussions de cours, vous voudrez très probablement restreindre la consultation aux seuls étudiants inscrits.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Require SSO To View Comments** activé, puis définissez le niveau de sécurité sur **Secure SSO** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Voir [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un domaine ou une page spécifique.

#### Moodle Gotchas

**FastComments missing from the activity chooser.** L’administrateur du site a enregistré l’outil mais n’a pas réglé **Tool configuration usage** sur **Show in activity chooser and as a preconfigured tool**. Corrigez cela sous **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icône d’engrenage sur la vignette FastComments.

**Launch fails or shows a blank frame when set to "Same window".** Les cookies de session de Moodle utilisent `SameSite=Lax` par défaut, et certains navigateurs les suppriment lors du POST intersite que LTI 1.3 utilise pour revenir depuis FastComments. Réglez **Launch container** sur **New window** pour l’activité. Il s’agit d’une exigence stricte pour les intégrations FastComments dans une Page ou un Book, car le chemin de lancement intégré depuis l’éditeur ouvre toujours une nouvelle fenêtre.

**The `iss` claim is the Moodle site URL, not a tenant ID.** FastComments utilise l’URL du site Moodle (la valeur de configuration `wwwroot`) comme émetteur LTI. Si votre instance Moodle déménage vers un nouveau domaine ou si vous changez `wwwroot`, les fils FastComments existants restent liés à l’ancien émetteur et ne correspondront pas aux nouveaux lancements. Réenregistrez l’outil pour la nouvelle URL et migrez les fils via l’administration FastComments si nécessaire.

**Activity backup and restore.** Sauvegarder un cours et le restaurer dans un nouveau cours crée de nouveaux resource link IDs, donc les activités FastComments restaurées commencent avec des fils vides. Le cours original conserve les fils d’origine. C’est le comportement attendu, pas un bogue.

**Moodle 4.5 TinyMCE default.** Moodle 4.5 est livré avec TinyMCE comme éditeur par défaut pour les nouvelles installations. Le bouton External tool se trouve sous le menu **More** (`...`) plutôt que dans la barre d’outils principale. Les anciens sites mis à niveau depuis 4.1 conservent Atto à moins qu’un administrateur n’ait changé le défaut.