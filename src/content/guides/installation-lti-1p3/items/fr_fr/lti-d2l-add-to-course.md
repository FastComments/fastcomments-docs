Cette page explique comment ajouter FastComments à un cours Brightspace après qu’un administrateur a enregistré l’outil et créé un déploiement. Si l’outil n’est pas pas encore enregistré, voir d’abord le guide d’enregistrement D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments intégré en tant que sujet d’unité dans Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments exécuté dans une unité Brightspace, affichant des commentaires en fil de discussion et un sélecteur de mentions @-nommé" />
</div>

Brightspace propose deux expériences de création de contenu : **Contenu classique** et la **Nouvelle expérience de contenu** (aussi appelée **Lessons**). Les deux donnent accès à FastComments, mais les chemins de menu diffèrent. Chaque section ci‑dessous couvre les deux cas quand ils divergent.

#### Repérer l'outil FastComments

L’outil FastComments apparaît à deux endroits dans l’éditeur de contenu d’un cours :

1. Le sélecteur d’activités, accessible depuis le bouton **Add Existing** d’un module/unité (étiqueté **Add Existing Activities** dans les anciennes versions de Brightspace). FastComments apparaît directement dans le sélecteur dans les versions récentes de Brightspace ; les versions plus anciennes le placent sous un sous‑menu **External Learning Tools**. Dans les deux cas, cela ajoute FastComments comme sujet autonome.
2. La boîte de dialogue **Insert Stuff** dans l’éditeur HTML, sous **LTI Advantage**. Cela intègre FastComments en ligne dans un sujet HTML via le flux de deep linking LTI.

Si FastComments n’apparaît dans aucun des deux sélecteurs, le déploiement n’est pas activé pour l’unité organisationnelle contenant le cours. Demandez à votre administrateur Brightspace d’ouvrir **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > outil FastComments > **View Deployments**, d’ouvrir le déploiement et d’ajouter l’unité organisationnelle du cours (ou une unité parent) sous **Org Units**.

#### Ajouter FastComments comme sujet dans un module

Classic Content :

1. Ouvrez le cours et cliquez sur **Content** dans la barre de navigation.
2. Sélectionnez le module qui doit contenir la discussion (ou créez‑en un via **Add a module**).
3. Cliquez sur **Add Existing** (Brightspace ancien : **Add Existing Activities** > **External Learning Tools**).
4. Dans le sélecteur, cliquez sur **FastComments**. Brightspace crée un sujet dans le module et vous ramène à la vue de contenu.
5. Cliquez sur le nouveau sujet. Renommez‑le en quelque chose de descriptif comme `FastComments Discussion` en utilisant l’éditeur de titre en ligne.

New Content Experience (Lessons) :

1. Ouvrez le cours et cliquez sur **Content**.
2. Ouvrez l’unité et la leçon qui doivent contenir la discussion.
3. Cliquez sur **Add** > **Existing Activity** et sélectionnez **FastComments** (Brightspace ancien : niché sous **External Learning Tools**).
4. L’activité est ajoutée à la leçon.
5. Cliquez sur le titre de l’activité pour le renommer.

La première fois qu’un utilisateur (enseignant ou étudiant) ouvre le sujet, FastComments initialise le fil pour ce resource link. Le fil est lié à l’ID du resource link, donc renommer ou déplacer le sujet ne change pas le fil chargé.

#### Intégrer FastComments en ligne dans un sujet HTML

Utilisez ce flux lorsque vous souhaitez que les commentaires apparaissent sous une lecture, une vidéo ou autre contenu à l’intérieur de la même page de sujet plutôt que comme sujet séparé.

1. Ouvrez ou créez un sujet HTML dans le module/la leçon.
2. Cliquez sur **Edit HTML** pour ouvrir l’éditeur HTML de Brightspace.
3. Placez le curseur à l’endroit où le fil de commentaires doit apparaître.
4. Cliquez sur le bouton **Insert Stuff** (icône en forme de pièce de puzzle dans la barre d’outils de l’éditeur).
5. Dans la boîte Insert Stuff, faites défiler jusqu’à **LTI Advantage** et cliquez sur **FastComments**.
6. FastComments ouvre un sélecteur de deep linking. Confirmez l’emplacement (les options par défaut conviennent pour les discussions de contenu) ; cliquez sur **Insert** ou **Continue**.
7. Brightspace revient à l’éditeur HTML avec un bloc substitut représentant le lancement LTI. Cliquez sur **Save and Close** du sujet.

Quand le sujet se charge, Brightspace remplace le substitut par un iframe qui lance automatiquement FastComments via LTI. Les étudiants voient le fil de discussion en ligne.

Un seul sujet HTML peut contenir plusieurs intégrations FastComments en deep linking. Chaque intégration reçoit son propre fil parce que chaque deep link produit un resource link ID distinct.

#### Sujet de module vs lien rapide intégré

Choisissez l’approche **sujet de module** lorsque :

- La discussion est l’activité principale pour cette étape du module.
- Vous souhaitez que le sujet apparaisse dans la table des matières de Brightspace, le suivi d’achèvement et Class Progress.

Choisissez l’approche **intégration en ligne** lorsque :

- Les commentaires doivent se trouver sous d’autres contenus sur la même page.
- Vous ne voulez pas d’un élément distinct traçable pour l’achèvement dans la table des matières.

#### Visibilité, brouillon et conditions de publication

Un nouveau sujet FastComments est visible par défaut pour les étudiants. Pour le masquer pendant la configuration :

1. Dans l’éditeur de contenu, cliquez sur le titre du sujet (Classic) ou sur le menu à trois points de l’activité (New Content Experience).
2. Définissez le statut sur **Draft** (Classic) ou basculez **Visibility** sur off (New Content Experience).

Les sujets en brouillon sont invisibles pour les étudiants. Les enseignants et TA les voient toujours avec un badge « Draft ».

Pour restreindre le sujet à un groupe ou une section spécifique :

1. Ouvrez le sujet.
2. Cliquez sur le menu du titre du sujet > **Edit Properties In-place** (Classic) ou **Edit** > **Restrictions** (New Content Experience).
3. Sous **Release Conditions**, cliquez sur **Create**.
4. Choisissez **Group enrollment** ou **Section enrollment**, sélectionnez le groupe/la section, et enregistrez.

Les conditions de publication s’ajoutent au mappage des rôles propre à FastComments. Les étudiants qui ne peuvent pas voir le sujet n’obtiennent pas de lancement LTI.

#### Ce que voient les étudiants au premier lancement

Lorsqu’un étudiant clique sur le sujet (ou charge un sujet HTML contenant une intégration) :

1. Brightspace effectue le lancement LTI 1.3 en arrière‑plan.
2. FastComments reçoit le nom de l’étudiant, son email, l’URL de son avatar et son rôle LMS, et le connecte automatiquement. Il n’y a pas d’invite de connexion FastComments.
3. Le fil de commentaires pour ce resource link se rend à l’intérieur de l’iframe Brightspace.

Mappage des rôles au lancement :

- Brightspace `Administrator` devient un administrateur FastComments (admin) pour le fil (modération complète, suppression, bannissement et accès à la configuration).
- Brightspace `Instructor` devient un modérateur FastComments (moderator) (épingler, masquer, supprimer, bannir).
- Tous les autres rôles (`Learner`, `TeachingAssistant`, etc.) deviennent des commentateurs standards.

Les commentaires sont attribués au compte Brightspace de l’étudiant. Si l’étudiant modifie son nom ou son avatar dans Brightspace, le prochain lancement LTI synchronise le changement.

#### Hauteur de l’iframe et redimensionnement

FastComments émet le postMessage `org.imsglobal.lti.frameResize` à chaque rendu du fil et lors des changements de contenu (nouveau commentaire, développement des réponses). Brightspace écoute ce message et ajuste la hauteur de l’iframe pour que le fil ne soit pas tronqué et n’affiche pas de barre de défilement interne.

Si l’iframe reste à une hauteur fixe et trop courte :

- Confirmez que le cours est chargé en HTTPS. L’écouteur postMessage de Brightspace rejette les frames en contenu mixte.
- Confirmez qu’aucune extension de navigateur ne bloque le canal postMessage.
- Pour les intégrations en ligne dans un sujet HTML, le HTML environnant ne doit pas envelopper l’iframe dans un conteneur à hauteur fixe. Supprimez tout `style="height: ..."` en ligne de l’élément parent.

#### Pièges spécifiques à Brightspace

**L’outil n’apparaît pas dans le sélecteur Add Existing.** Le déploiement n’est pas activé pour l’unité organisationnelle de ce cours. Un administrateur doit ajouter l’unité organisationnelle (ou une unité parente) à la liste **Org Units** du déploiement. L’enregistrement de l’outil seul ne suffit pas ; le déploiement définit les cours qui voient l’outil.

**`deployment_id` ne correspond pas au lancement.** FastComments ancre en TOFU le premier `deployment_id` rencontré pour une registration. Si un administrateur supprime le déploiement initial et en crée un nouveau, les lancements depuis le nouveau déploiement sont rejetés avec une erreur de mismatch de déploiement. La solution est de re‑enregistrer FastComments (générer une nouvelle URL d’enregistrement et relancer l’enregistrement dynamique) ; l’ancien enregistrement de configuration est remplacé.

**L’outil se lance mais affiche « Invalid LTI launch ».** Le cours se trouve dans une structure de locataire/organisation différente de celle couverte par le déploiement, ou le déploiement a été désactivé après l’enregistrement. Revérifiez **Admin Tools** > **Manage Extensibility** > **LTI Advantage** > FastComments > bascule **Enabled** et la liste des unités organisationnelles du déploiement.

**Noms et rôles manquants dans FastComments.** Brightspace envoie des lancements LTI avec les claims Names and Role Provisioning Services (NRPS). Si un cours a été mis à niveau depuis un ancien lien LTI 1.1, le lancement peut ne pas contenir les claims `name` et `email`. Réajoutez le sujet FastComments via **Add Existing** (ne migrez pas l’ancien lien) pour que le lancement utilise LTI 1.3.

**L’intégration affiche un écran de connexion au lieu de l’auto‑SSO.** Le sujet HTML a été inséré en tant que simple `<iframe>` pointant vers FastComments plutôt que via **Insert Stuff** > **LTI Advantage**. Les iframes simples contournent le lancement LTI et amènent les utilisateurs sur la page publique FastComments. Supprimez l’iframe et réinsérez‑la via le flux Insert Stuff.