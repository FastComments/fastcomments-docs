Ce guide couvre l'ajout de FastComments à un cours Moodle 4.x après qu'un administrateur du site a enregistré l'outil et l'a configuré pour s'afficher dans le sélecteur d'activités. Si FastComments n'est pas encore enregistré, consultez d'abord le guide d'enregistrement Moodle.

#### Ouvrir le cours en mode édition

1. Connectez-vous à Moodle en tant qu'Enseignant (avec droits d'édition) (ou rôle supérieur) pour le cours.
2. Ouvrez le cours.
3. Activez le **Mode édition** en utilisant l'interrupteur en haut à droite de l'en-tête du cours.

Moodle 4.x a remplacé le menu déroulant héritage "Add an activity or resource" utilisé en 3.x par une boîte de dialogue de sélection d'activités en plein écran. Moodle 4.5 conserve ce même sélecteur mais ajoute une ligne d'épingles/favoris en haut, donc épingler FastComments une fois le rendra plus rapide d'accès dans les sections suivantes.

#### Ajouter l'activité FastComments

1. Faites défiler jusqu'à la section du cours (thème ou semaine) où la discussion doit se trouver.
2. Cliquez sur **Add an activity or resource** en bas de cette section.
3. Dans la boîte de dialogue du sélecteur, sélectionnez **FastComments**. Si vous ne le voyez pas, passez à la section des pièges ci-dessous.

Le formulaire de configuration de l'activité s'ouvre. Les champs importants :

- **Activity name** (obligatoire). S'affiche sur la page du cours et dans le carnet de notes. Exemple : `Week 3 Discussion`.
- **Activity description**. Texte d'introduction facultatif affiché au-dessus du fil de commentaires.
- **Show description on course page**. Cochez ceci si vous voulez que la description soit visible sans entrer dans l'activité.
- **Preconfigured tool**. Régler sur `FastComments` (sélectionné automatiquement lorsqu'il est lancé depuis le sélecteur). Ne pas modifier.
- **Launch container**. Régler sur **New window**. Voir la section des pièges pour expliquer pourquoi "Same window" casse dans certaines installations Moodle.
- **Tool URL**, **Public key**, **Shared secret**, **Custom parameters**. Laisser vide. L'enregistrement dynamique a géré ces éléments au niveau du site.

Faites défiler vers le bas et cliquez sur **Save and return to course** (ou **Save and display** pour ouvrir l'activité immédiatement).

L'activité apparaît comme une ligne dans la section avec l'icône FastComments. Les étudiants cliquent sur la ligne pour ouvrir le fil de commentaires.

#### Intégrer FastComments inline avec l'éditeur

Pour un fil à l'intérieur d'une Page, d'un chapitre de Book, d'une Leçon, ou de toute autre ressource utilisant l'éditeur Atto ou TinyMCE :

1. Ouvrez la ressource en mode édition.
2. Placez le curseur à l'endroit où le fil doit apparaître.
3. Dans la barre d'outils de l'éditeur, cliquez sur le bouton **LTI** / **External tool**. Dans Atto, il est étiqueté "Insert LTI Advantage content". Dans TinyMCE (par défaut dans Moodle 4.3+), il se trouve sous le menu **More** en tant que **External tools**.
4. Choisissez **FastComments** dans la liste des outils.
5. FastComments ouvre un sélecteur de liens profonds. Confirmez le titre du fil et cliquez sur **Embed**.
6. L'éditeur insère un bloc de remplacement LTI. Enregistrez la ressource.

Chaque instance intégrée est un fil distinct identifié par l'ID d'élément de contenu de lien profond, donc une Page avec trois intégrations FastComments aura trois fils indépendants.

#### Restrictions d'accès et paramètres de groupe

Les paramètres d'activité standard de Moodle s'appliquent aux activités FastComments :

- **Common module settings** > **Group mode**. Mettre ceci sur **Separate groups** ou **Visible groups** ne divise pas automatiquement FastComments en fils par groupe. Le mode de groupe de Moodle ne fait que filtrer le carnet de notes et la liste des membres. Pour exécuter un fil distinct par groupe, ajoutez une activité FastComments par groupe et utilisez **Restrict access** pour limiter chacune.
- **Restrict access** > **Add restriction**. Prend en charge les conditions standard de Moodle : **Date**, **Grade**, **Group**, **Grouping**, **User profile**, et ensembles de restrictions imbriqués. Utilisez **Group** pour verrouiller une activité FastComments sur un seul groupe.
- **Activity completion**. Régler sur **Students must view this activity to complete it** si vous voulez le suivi d'achèvement. FastComments ne rapporte pas actuellement d'événement d'achèvement à Moodle au-delà du lancement.

#### Mappage des rôles

FastComments lit la réclamation LTI `roles` que Moodle envoie à chaque lancement et la mappe comme suit :

- Moodle **Gestionnaire** ou **Administrateur du site** -> FastComments **admin**
- Moodle **Enseignant (avec droits d'édition)** ou **Enseignant sans droits d'édition** -> FastComments **moderator**
- Moodle **Étudiant** -> FastComments **commenter**
- Moodle **Invité** -> accès en lecture seule

Les administrateurs peuvent supprimer n'importe quel commentaire, bannir des utilisateurs et modifier les paramètres du fil. Les modérateurs peuvent supprimer et approuver des commentaires à l'intérieur du fil dans lequel ils ont lancé l'outil. Les rôles Moodle personnalisés héritent du mappage de l'archétype à partir duquel ils ont été clonés.

#### Ce que voient les étudiants

Les étudiants cliquent sur l'activité FastComments (ou font défiler jusqu'au bloc intégré à l'intérieur d'une Page ou d'un Book). Moodle envoie leur identité à FastComments via le lancement LTI :

- Pas d'écran de connexion. FastComments les connecte en utilisant le compte Moodle.
- Leur nom d'affichage, leur e-mail et leur avatar proviennent de Moodle.
- Le fil est scellé sur (Moodle site, course, resource link ID), donc la même activité dupliquée dans un autre cours obtient un fil neuf.
- Les réponses en arborescence, le vote et les notifications fonctionnent de la même manière que pour un fil FastComments autonome.

#### Pièges Moodle

**FastComments absent du sélecteur d'activités.** L'administrateur du site a enregistré l'outil mais n'a pas réglé **Tool configuration usage** sur **Show in activity chooser and as a preconfigured tool**. Corrigez cela sous **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools** > icône d'engrenage sur la tuile FastComments.

**Le lancement échoue ou affiche une fenêtre vide lorsqu'il est réglé sur "Same window".** Les cookies de session de Moodle utilisent `SameSite=Lax` par défaut, et certains navigateurs les suppriment lors du POST cross-site que LTI 1.3 utilise pour revenir depuis FastComments. Réglez **Launch container** sur **New window** pour l'activité. C'est une exigence stricte pour les intégrations FastComments à l'intérieur d'une Page ou d'un Book, car le chemin de lancement intégré dans l'éditeur ouvre toujours une nouvelle fenêtre.

**La réclamation `iss` est l'URL du site Moodle, pas un ID de locataire.** FastComments utilise l'URL du site Moodle (la valeur de configuration `wwwroot`) comme émetteur LTI. Si votre instance Moodle migre vers un nouveau domaine ou si vous changez `wwwroot`, les fils FastComments existants restent liés à l'ancien émetteur et ne correspondront pas aux nouveaux lancements. Réenregistrez l'outil pour la nouvelle URL et migrez les fils via l'administration FastComments si nécessaire.

**Sauvegarde et restauration d'activité.** Sauvegarder un cours et le restaurer dans un nouveau cours crée de nouveaux resource link IDs, donc les activités FastComments restaurées commencent avec des fils vides. Le cours d'origine conserve les fils originaux. C'est le comportement attendu, pas un bug.

**TinyMCE par défaut dans Moodle 4.5.** Moodle 4.5 est livré avec TinyMCE comme éditeur par défaut pour les nouvelles installations. Le bouton External tool se trouve sous le menu **More** (`...`) plutôt que dans la barre d'outils principale. Les sites plus anciens mis à niveau depuis 4.1 conservent Atto à moins qu'un administrateur n'ait changé le paramètre par défaut.