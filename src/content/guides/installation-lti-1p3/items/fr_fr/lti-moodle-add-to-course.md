Ce guide explique comment ajouter FastComments à un cours Moodle 4.x après qu'un administrateur du site a enregistré l'outil et l'a configuré pour qu'il apparaisse dans le sélecteur d'activités. Si FastComments n'est pas encore enregistré, consultez d'abord le guide d'enregistrement pour Moodle.

#### Open the Course in Edit Mode

1. Connectez-vous à Moodle en tant qu'Enseignant avec droits d'édition (ou rôle supérieur) pour le cours.
2. Ouvrez le cours.
3. Activez le **mode édition** à l'aide du commutateur dans le coin supérieur droit de l'en-tête du cours.

Moodle 4.x a remplacé le menu déroulant hérité "Add an activity or resource" utilisé dans la 3.x par une boîte de sélection d'activités en plein écran. Moodle 4.5 conserve la même boîte de sélection mais ajoute une rangée d'étoiles/favoris en haut, donc épingler FastComments une fois le rendra plus rapide à retrouver dans les sections suivantes.

#### Add the FastComments Activity

1. Descendez jusqu'à la section du cours (thème ou semaine) où la discussion doit se trouver.
2. Cliquez sur **Add an activity or resource** au bas de cette section.
3. Dans la boîte de sélection, sélectionnez **FastComments**. Si vous ne le voyez pas, passez à la section des pièges (gotchas) ci-dessous.

Le formulaire de paramètres de l'activité s'ouvre. Les champs importants :

- **Activity name** (obligatoire). S'affiche sur la page du cours et dans le carnet de notes. Exemple : `Week 3 Discussion`.
- **Activity description**. Texte d'introduction optionnel rendu au-dessus du fil de commentaires.
- **Show description on course page**. Cochez ceci si vous souhaitez que la description soit visible sans ouvrir l'activité.
- **Preconfigured tool**. Réglez sur `FastComments` (sélectionné automatiquement lors du lancement depuis la boîte de sélection). Ne changez pas.
- **Launch container**. Réglez sur **New window**. Voir la section des pièges pour comprendre pourquoi "Same window" casse dans certaines installations Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Laissez vides. L'enregistrement dynamique a géré ces éléments au niveau du site.

Descendez en bas et cliquez sur **Save and return to course** (ou **Save and display** pour ouvrir l'activité immédiatement).

L'activité apparaît comme une ligne dans la section avec l'icône FastComments. Les étudiants cliquent sur la ligne pour ouvrir le fil de commentaires.

#### Embed FastComments Inline with the Editor

Pour insérer un fil dans une Page, un chapitre de Book, une Leçon ou toute autre ressource qui utilise l'éditeur Atto ou TinyMCE :

1. Ouvrez la ressource en mode édition.
2. Placez le curseur à l'endroit où le fil doit apparaître.
3. Dans la barre d'outils de l'éditeur, cliquez sur le bouton **LTI** / **External tool**. Dans Atto il est étiqueté "Insert LTI Advantage content". Dans TinyMCE (par défaut dans Moodle 4.3+) il se trouve sous le menu **More** comme **External tools**.
4. Choisissez **FastComments** dans la liste des outils.
5. FastComments ouvre un sélecteur de deep-linking. Confirmez le titre du fil et cliquez sur **Embed**.
6. L'éditeur insère un bloc de substitution LTI. Enregistrez la ressource.

Chaque instance intégrée est un fil distinct identifié par l'ID de l'élément de contenu deep-link, donc une Page avec trois incorporations FastComments crée trois fils indépendants.

#### Restrict Access and Group Settings

Les paramètres standard d'activité Moodle s'appliquent aux activités FastComments :

- **Common module settings** > **Group mode**. Le fait de définir ceci sur **Separate groups** ou **Visible groups** ne sépare pas automatiquement FastComments en fils par groupe. Le mode de groupe de Moodle ne filtre que le carnet de notes et la liste des membres. Pour gérer un fil séparé par groupe, ajoutez une activité FastComments par groupe et utilisez **Restrict access** pour limiter chacune.
- **Restrict access** > **Add restriction**. Prend en charge les conditions standard Moodle : **Date**, **Grade**, **Group**, **Grouping**, **User profile**, et des ensembles de restrictions imbriqués. Utilisez **Group** pour verrouiller une activité FastComments sur un seul groupe.
- **Activity completion**. Réglez sur **Students must view this activity to complete it** si vous voulez le suivi de complétion. FastComments ne rapporte actuellement pas d'événement de complétion à Moodle au-delà du lancement.

#### Role Mapping

FastComments lit la revendication LTI `roles` que Moodle envoie à chaque lancement et la mappe comme suit :

- Moodle **Gestionnaire** ou **Administrateur du site** -> FastComments **admin**
- Moodle **Enseignant (avec droits d'édition)** ou **Enseignant non éditeur** -> FastComments **moderator**
- Moodle **Étudiant** -> FastComments **commenter**
- Moodle **Invité** -> lecture seule

Les administrateurs peuvent supprimer n'importe quel commentaire, bannir des utilisateurs et modifier les paramètres du fil. Les modérateurs peuvent supprimer et approuver des commentaires dans le fil dans lequel ils ont été lancés. Les rôles Moodle personnalisés héritent du mapping de l'archétype dont ils ont été clonés.

#### What Students See

Les étudiants cliquent sur l'activité FastComments (ou descendent jusqu'au bloc intégré dans une Page ou un Book). Moodle envoie leur identité à FastComments via le lancement LTI :

- Pas d'écran de connexion. FastComments les connecte en utilisant le compte Moodle.
- Leur nom affiché, adresse e-mail et avatar proviennent de Moodle.
- Le fil est scoppé sur `(Moodle site, course, resource link ID)`, donc la même activité dupliquée dans un autre cours obtient un fil neuf.
- Les réponses imbriquées, les votes et les notifications fonctionnent de la même manière qu'un fil FastComments autonome.

#### Lock Down Public Access (Recommended)

Par défaut, les données de commentaires FastComments sont lisibles publiquement. Toute personne pouvant deviner l'URL d'un fil ou un endpoint API peut voir ses commentaires, même en dehors de Moodle. Pour les discussions de cours, vous voulez très probablement restreindre la lecture aux seuls étudiants inscrits.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Require SSO To View Comments** activé, puis définissez le niveau de sécurité sur **Secure SSO** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Voir [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou une page.

#### Moodle Gotchas

**FastComments absent du sélecteur d'activités.** L'administrateur du site a enregistré l'outil mais n'a pas défini **Tool configuration usage** sur **Show in activity chooser and as a preconfigured tool**. Corrigez cela sous **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icône d'engrenage sur la tuile FastComments.

**Le lancement échoue ou affiche un cadre vide lorsque réglé sur "Same window".** Les cookies de session de Moodle utilisent `SameSite=Lax` par défaut, et certains navigateurs les suppriment lors du POST cross-site que LTI 1.3 utilise pour revenir de FastComments. Réglez **Launch container** sur **New window** pour l'activité. C'est une exigence stricte pour les intégrations FastComments dans une Page ou un Book, puisque le chemin de lancement intégré par l'éditeur ouvre toujours une nouvelle fenêtre.

**La revendication `iss` est l'URL du site Moodle, pas un ID de locataire.** FastComments utilise l'URL du site Moodle (la valeur de configuration `wwwroot`) comme émetteur LTI. Si votre instance Moodle déménage vers un nouveau domaine ou si vous changez `wwwroot`, les fils FastComments existants restent liés à l'ancien émetteur et ne correspondront pas aux nouveaux lancements. Réenregistrez l'outil contre la nouvelle URL et migrez les fils via l'administration FastComments si nécessaire.

**Sauvegarde et restauration d'activité.** Sauvegarder un cours et le restaurer dans un nouveau cours crée de nouveaux IDs de lien de ressource, donc les activités FastComments restaurées commencent avec des fils vides. Le cours d'origine conserve les fils originaux. C'est le comportement attendu, pas un bug.

**TinyMCE par défaut dans Moodle 4.5.** Moodle 4.5 est livré avec TinyMCE comme éditeur par défaut pour les nouvelles installations. L'emplacement du bouton External tool se trouve sous le menu **More** (`...`) plutôt que dans la barre d'outils principale. Les sites plus anciens qui ont été mis à niveau depuis la 4.1 conservent Atto à moins qu'un administrateur n'ait changé le défaut.