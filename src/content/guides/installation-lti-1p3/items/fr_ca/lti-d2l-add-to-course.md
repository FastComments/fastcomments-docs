Cette page couvre l’ajout de FastComments à un cours Brightspace après qu’un administrateur a enregistré l’outil et créé un déploiement. Si l’outil n’est pas encore enregistré, consultez d’abord le guide d’enregistrement D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments intégré comme sujet d’unité dans Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments exécuté à l’intérieur d’une unité Brightspace, affichant des commentaires en fil de discussion et un sélecteur de mention @-nom" />
</div>

Brightspace propose deux expériences de création de contenu : **Classic Content** et la **New Content Experience** (aussi appelée **Lessons**). Les deux exposent FastComments, mais les chemins de menu diffèrent. Chaque section ci‑dessous couvre les deux cas là où ils divergent.

#### Locate the FastComments Tool

L’outil FastComments apparaît à deux endroits dans un éditeur de contenu de cours :

1. Le sélecteur d’activité, accessible depuis le bouton **Add Existing** d’un module/unité (étiqueté **Add Existing Activities** dans les anciennes versions de Brightspace). FastComments apparaît directement dans le sélecteur dans les versions récentes de Brightspace ; les anciennes versions l’imbriquent sous un sous‑menu **External Learning Tools**. Chacune des deux voies ajoute FastComments en tant que sujet autonome.
2. La boîte de dialogue **Insert Stuff** à l’intérieur de l’éditeur HTML, sous **LTI Advantage**. Ceci intègre FastComments en ligne dans un sujet HTML via le flux de deep linking LTI.

Si FastComments n’apparaît dans aucun des deux sélecteurs, le déploiement n’est pas activé pour l’org unit contenant le cours. Demandez à votre administrateur Brightspace d’ouvrir **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments tool > **View Deployments**, d’ouvrir le déploiement et d’ajouter l’org unit du cours (ou une org unit parente) sous **Org Units**.

#### Add FastComments as a Topic in a Module

Classic Content:

1. Ouvrez le cours et cliquez sur **Content** dans la barre de navigation.
2. Sélectionnez le module qui doit contenir la discussion (ou créez‑en un via **Add a module**).
3. Cliquez sur **Add Existing** (Brightspace plus ancien : **Add Existing Activities** > **External Learning Tools**).
4. Dans le sélecteur, cliquez sur **FastComments**. Brightspace crée un sujet dans le module et vous ramène à la vue de contenu.
5. Cliquez sur le nouveau sujet. Renommez‑le par quelque chose de descriptif comme `FastComments Discussion` en utilisant l’éditeur de titre en ligne.

New Content Experience (Lessons):

1. Ouvrez le cours et cliquez sur **Content**.
2. Ouvrez l’unité et la leçon qui doivent contenir la discussion.
3. Cliquez sur **Add** > **Existing Activity** et sélectionnez **FastComments** (Brightspace plus ancien : imbriqué sous **External Learning Tools**).
4. L’activité est ajoutée à la leçon.
5. Cliquez sur le titre de l’activité pour le renommer.

La première fois qu’un utilisateur (enseignant ou étudiant) ouvre le sujet, FastComments initialise le fil pour ce resource link. Le fil est lié à l’ID du resource link, donc renommer ou déplacer le sujet ne change pas le fil chargé.

#### Embed FastComments Inline in an HTML Topic

Utilisez ce flux lorsque vous voulez que les commentaires apparaissent sous une lecture, une vidéo ou autre contenu à l’intérieur de la même page de sujet plutôt que comme sujet séparé.

1. Ouvrez ou créez un sujet HTML dans le module/la leçon.
2. Cliquez sur **Edit HTML** pour ouvrir l’éditeur HTML de Brightspace.
3. Placez le curseur à l’endroit où le fil de commentaires doit apparaître.
4. Cliquez sur le bouton **Insert Stuff** (icône en forme de pièce de puzzle dans la barre d’outils de l’éditeur).
5. Dans la boîte Insert Stuff, faites défiler jusqu’à **LTI Advantage** et cliquez sur **FastComments**.
6. FastComments ouvre un sélecteur de deep linking. Confirmez l’emplacement (les options par défaut conviennent pour les discussions de contenu) ; cliquez sur **Insert** ou **Continue**.
7. Brightspace revient à l’éditeur HTML avec un bloc de remplacement représentant le lancement LTI. Cliquez sur **Save and Close** sur le sujet.

Lorsque le sujet se charge, Brightspace remplace le placeholder par un iframe qui lance automatiquement FastComments via LTI. Les étudiants voient le fil de discussion en ligne.

Un seul sujet HTML peut contenir plusieurs intégrations deep‑linked FastComments. Chaque intégration obtient son propre fil car chaque deep link produit un resource link ID distinct.

#### Module Topic vs Inline Quicklink

Choisissez l’approche **module topic** lorsque :

- La discussion est l’activité principale pour cette étape du module.
- Vous voulez que le sujet apparaisse dans la table des matières de Brightspace, le suivi de complétion et Class Progress.

Choisissez l’approche **inline embed** lorsque :

- Les commentaires doivent être sous d’autres contenus sur la même page.
- Vous ne voulez pas d’élément séparé suivi pour la complétion dans la table des matières.

#### Visibility, Draft, and Release Conditions

Un nouveau sujet FastComments est visible par les étudiants par défaut. Pour le masquer pendant la configuration :

1. Dans l’éditeur de contenu, cliquez sur le titre du sujet (Classic) ou sur le menu à trois points sur l’activité (New Content Experience).
2. Définissez le statut sur **Draft** (Classic) ou basculez **Visibility** sur off (New Content Experience).

Les sujets Draft sont invisibles aux étudiants. Les instructeurs et TA les voient toujours avec un badge « Draft ».

Pour restreindre le sujet à un groupe ou une section spécifique :

1. Ouvrez le sujet.
2. Cliquez sur le menu du titre du sujet > **Edit Properties In-place** (Classic) ou **Edit** > **Restrictions** (New Content Experience).
3. Sous **Release Conditions**, cliquez sur **Create**.
4. Choisissez **Group enrollment** ou **Section enrollment**, sélectionnez le groupe/la section, et enregistrez.

Les conditions de publication s’additionnent avec le propre mapping de rôles de FastComments. Les étudiants qui ne peuvent pas voir le sujet ne reçoivent pas de lancement LTI.

#### What Students See on First Launch

Lorsque qu’un étudiant clique sur le sujet (ou charge un sujet HTML avec une intégration) :

1. Brightspace effectue le lancement LTI 1.3 en arrière‑plan.
2. FastComments reçoit le nom de l’étudiant, son courriel, l’URL de son avatar et le rôle LMS, et le connecte automatiquement. Il n’y a pas d’invite de connexion FastComments.
3. Le fil de commentaires pour ce resource link se rend à l’intérieur de l’iframe Brightspace.

Mapping des rôles au lancement :

- Brightspace `Administrator` devient un FastComments **admin** pour le fil (modération complète, suppression, bannissement et accès à la configuration).
- Brightspace `Instructor` devient un FastComments **moderator** (épingler, masquer, supprimer, bannir).
- Tous les autres rôles (`Learner`, `TeachingAssistant`, etc.) deviennent des commentateurs standards.

Les commentaires sont attribués au compte Brightspace de l’étudiant. Si l’étudiant modifie son nom ou son avatar dans Brightspace, le prochain lancement LTI synchronise la modification.

#### Iframe Height and Resize

FastComments émet le postMessage `org.imsglobal.lti.frameResize` à chaque rendu de fil et lors des changements de contenu (nouveau commentaire, déploiement des réponses). Brightspace écoute ce message et ajuste la hauteur de l’iframe afin que le fil ne soit pas tronqué et qu’aucune barre de défilement interne n’apparaisse.

Si l’iframe reste à une hauteur fixe et courte :

- Confirmez que le cours est chargé via HTTPS. Le listener postMessage de Brightspace rejette les frames en contenu mixte.
- Confirmez qu’aucune extension de navigateur ne bloque le canal postMessage.
- Pour les intégrations inline dans un sujet HTML, le HTML environnant ne doit pas envelopper l’iframe dans un conteneur à hauteur fixe. Supprimez tout style inline `style="height: ..."` de l’élément parent.

#### Brightspace-Specific Gotchas

**Tool not showing in the Add Existing picker.** Le déploiement n’est pas activé pour l’org unit de ce cours. Un administrateur doit ajouter l’org unit (ou une parente) à la liste **Org Units** du déploiement. L’enregistrement de l’outil seul ne suffit pas ; le déploiement détermine quels cours voient l’outil.

**`deployment_id` mismatch on launch.** FastComments « TOFU‑pins » le premier `deployment_id` qu’il voit pour un enregistrement. Si un administrateur supprime le déploiement initial et en crée un nouveau, les lancements depuis le nouveau déploiement sont rejetés avec une erreur de non‑correspondance de déploiement. La solution consiste à réenregistrer FastComments (générez une nouvelle URL d’enregistrement (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez‑la ici</a>) et relancez la Dynamic Registration) ; l’ancien enregistrement de configuration est remplacé.

**Tool launches but shows "Invalid LTI launch".** Le cours est dans une structure de tenant/org différente de celle couverte par le déploiement, ou le déploiement a été désactivé après l’enregistrement. Revérifiez **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > bascule **Enabled** et la liste des org units du déploiement.

**Names and roles missing inside FastComments.** Brightspace envoie des lancements LTI avec les claims NRPS (Names and Role Provisioning Services). Si un cours a été mis à niveau depuis un ancien lien LTI 1.1, le lancement peut ne pas contenir les claims `name` et `email`. Réajoutez le sujet FastComments via **Add Existing** (ne migrez pas l’ancien lien) afin que le lancement utilise LTI 1.3.

**Embed shows a login screen instead of auto-SSO.** Le sujet HTML a été inséré comme un simple `<iframe>` pointant vers FastComments plutôt que via **Insert Stuff** > **LTI Advantage**. Les iframes simples contournent le lancement LTI et amènent les utilisateurs sur la page publique FastComments. Supprimez l’iframe et réinsérez‑la via le flux Insert Stuff.