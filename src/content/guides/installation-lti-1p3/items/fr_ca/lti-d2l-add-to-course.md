Cette page explique comment ajouter FastComments à un cours Brightspace après qu’un administrateur a enregistré l’outil et créé un déploiement. Si l’outil n’est pas encore enregistré, consultez d’abord le guide d’enregistrement D2L.

<div class="screenshot white-bg">
    <div class="title">FastComments intégré comme sujet d’unité dans Brightspace</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-d2l-comments-in-unit.png" alt="FastComments s'exécutant à l'intérieur d'une unité Brightspace, affichant des commentaires en fil et un sélecteur de mentions @" />
</div>

Brightspace propose deux expériences de création de contenu : **Contenu classique** et la **Nouvelle expérience de contenu** (aussi appelée **Leçons**). Les deux exposent FastComments, mais les chemins de menu diffèrent. Chaque section ci-dessous couvre les deux là où ils divergent.

#### Localiser l’outil FastComments

L’outil FastComments apparaît à deux endroits dans l’éditeur de contenu d’un cours :

1. Le sélecteur d’activités, accessible depuis le bouton **Ajouter un élément existant** d’un module/une unité (étiqueté **Ajouter des activités existantes** dans les anciennes versions de Brightspace). FastComments apparaît directement dans le sélecteur dans les versions récentes de Brightspace ; dans les anciennes versions, il est imbriqué sous un sous-menu **Outils d’apprentissage externes**. Quel que soit le chemin, cela ajoute FastComments comme sujet autonome.
2. La boîte de dialogue **Insérer du contenu** dans l’éditeur HTML, sous **LTI Advantage**. Cela intègre FastComments en ligne dans un sujet HTML via le flux de liaison profonde LTI.

Si FastComments n’apparaît dans aucun des deux sélecteurs, le déploiement n’est pas activé pour l’unité organisationnelle contenant le cours. Demandez à votre administrateur Brightspace d’ouvrir **Outils d’administration** > **Gérer l’extensibilité** > **LTI Advantage** > outil FastComments > **Afficher les déploiements**, d’ouvrir le déploiement et d’ajouter l’unité organisationnelle du cours (ou une unité parente) sous **Unités d’organisation**.

#### Ajouter FastComments comme sujet dans un module

Contenu classique :

1. Ouvrez le cours et cliquez sur **Contenu** dans la barre de navigation.
2. Sélectionnez le module qui doit contenir la discussion (ou créez-en un via **Ajouter un module**).
3. Cliquez sur **Ajouter un élément existant** (Brightspace ancien : **Ajouter des activités existantes** > **Outils d’apprentissage externes**).
4. Dans le sélecteur, cliquez sur **FastComments**. Brightspace crée un sujet dans le module et vous renvoie à la vue de contenu.
5. Cliquez sur le nouveau sujet. Renommez-le avec quelque chose de descriptif comme `FastComments Discussion` en utilisant l’éditeur de titre en ligne.

Nouvelle expérience de contenu (Leçons) :

1. Ouvrez le cours et cliquez sur **Contenu**.
2. Ouvrez l’unité et la leçon qui doivent contenir la discussion.
3. Cliquez sur **Ajouter** > **Activité existante** et sélectionnez **FastComments** (Brightspace ancien : imbriqué sous **Outils d’apprentissage externes**).
4. L’activité est ajoutée à la leçon.
5. Cliquez sur le titre de l’activité pour le renommer.

La première fois qu’un utilisateur (enseignant ou étudiant) ouvre le sujet, FastComments initialise le fil pour ce lien de ressource. Le fil est lié à l’identifiant du lien de ressource, donc renommer ou déplacer le sujet ne change pas le fil chargé.

#### Intégrer FastComments en ligne dans un sujet HTML

Utilisez ce flux lorsque vous voulez que les commentaires apparaissent sous une lecture, une vidéo ou autre contenu à l’intérieur de la même page de sujet plutôt que comme un sujet distinct.

1. Ouvrez ou créez un sujet HTML dans le module/la leçon.
2. Cliquez sur **Modifier le HTML** pour ouvrir l’éditeur HTML de Brightspace.
3. Placez le curseur à l’endroit où le fil de commentaires doit apparaître.
4. Cliquez sur le bouton **Insérer du contenu** (icône de puzzle dans la barre d’outils de l’éditeur).
5. Dans la boîte Insérer du contenu, faites défiler jusqu’à **LTI Advantage** et cliquez sur **FastComments**.
6. FastComments ouvre un sélecteur de liaison profonde. Confirmez l’emplacement (les options par défaut conviennent pour les discussions de contenu) ; cliquez sur **Insérer** ou **Continuer**.
7. Brightspace revient à l’éditeur HTML avec un bloc de remplacement représentant le lancement LTI. Cliquez sur **Enregistrer et fermer** sur le sujet.

Lorsque le sujet se charge, Brightspace remplace le bloc de remplacement par une iframe qui lance automatiquement FastComments via LTI. Les étudiants voient le fil de discussion en ligne.

Un seul sujet HTML peut contenir plusieurs intégrations FastComments liées en profondeur. Chaque intégration obtient son propre fil car chaque lien profond produit un identifiant de lien de ressource distinct.

#### Sujet de module vs lien rapide intégré

Choisissez l’approche **sujet de module** lorsque :

- La discussion est l’activité principale pour cette étape du module.
- Vous voulez que le sujet apparaisse dans la table des matières de Brightspace, le suivi d’achèvement et Class Progress.

Choisissez l’approche **intégration en ligne** lorsque :

- Les commentaires doivent se trouver sous un autre contenu sur la même page.
- Vous ne voulez pas un élément distinct traçable pour l’achèvement dans la table des matières.

#### Visibilité, brouillon et conditions de publication

Un nouveau sujet FastComments est visible par les étudiants par défaut. Pour le masquer pendant sa configuration :

1. Dans l’éditeur de contenu, cliquez sur le titre du sujet (Contenu classique) ou sur le menu à trois points de l’activité (Nouvelle expérience de contenu).
2. Définissez le statut sur **Brouillon** (Contenu classique) ou désactivez la **Visibilité** (Nouvelle expérience de contenu).

Les sujets en brouillon sont invisibles pour les étudiants. Les enseignants et les assistants voient toujours ces sujets avec un badge « Brouillon ».

Pour restreindre le sujet à un groupe ou une section spécifique :

1. Ouvrez le sujet.
2. Cliquez sur le menu du titre du sujet > **Modifier les propriétés en place** (Contenu classique) ou **Modifier** > **Restrictions** (Nouvelle expérience de contenu).
3. Sous **Conditions de publication**, cliquez sur **Créer**.
4. Choisissez **Inscription à un groupe** ou **Inscription à une section**, sélectionnez le groupe/la section, et enregistrez.

Les conditions de publication s’empilent avec le propre mappage de rôles de FastComments. Les étudiants qui ne peuvent pas voir le sujet ne reçoivent pas de lancement LTI.

#### Ce que voient les étudiants lors du premier lancement

Quand un étudiant clique sur le sujet (ou charge un sujet HTML avec une intégration) :

1. Brightspace effectue le lancement LTI 1.3 en arrière-plan.
2. FastComments reçoit le nom de l’étudiant, l’email, l’URL de l’avatar et le rôle LMS, et les connecte automatiquement. Il n’y a pas d’invite de connexion FastComments.
3. Le fil de commentaires pour ce lien de ressource s’affiche dans l’iframe Brightspace.

Mappage des rôles au lancement :

- Brightspace `Administrator` devient un administrateur FastComments **admin** pour le fil (pleins pouvoirs de modération, suppression, bannissement et accès à la configuration).
- Brightspace `Instructor` devient un modérateur FastComments **moderator** (épingler, masquer, supprimer, bannir).
- Tous les autres rôles (`Learner`, `TeachingAssistant`, etc.) deviennent des commentateurs standards.

Les commentaires sont attribués au compte Brightspace de l’étudiant. Si l’étudiant modifie son nom ou son avatar dans Brightspace, le prochain lancement LTI synchronise la modification.

#### Hauteur de l’iframe et redimensionnement

FastComments émet le postMessage `org.imsglobal.lti.frameResize` à chaque rendu de fil et lors des changements de contenu (nouveau commentaire, développement des réponses). Brightspace écoute ce message et ajuste la hauteur de l’iframe afin que le fil ne soit pas coupé et qu’aucune barre de défilement interne ne s’affiche.

Si l’iframe reste à une hauteur fixe et trop courte :

- Confirmez que le cours est chargé via HTTPS. Le listener postMessage de Brightspace rejette les frames en contenu mixte.
- Confirmez qu’aucune extension de navigateur ne bloque le canal postMessage.
- Pour les intégrations en ligne dans un sujet HTML, le HTML environnant ne doit pas envelopper l’iframe dans un conteneur à hauteur fixe. Retirez tout `style="height: ..."` en ligne de l’élément parent.

#### Particularités spécifiques à Brightspace

**L’outil n’apparaît pas dans le sélecteur Ajouter un élément existant.** Le déploiement n’est pas activé pour l’unité organisationnelle de ce cours. Un administrateur doit ajouter l’unité organisationnelle (ou une unité parente) à la liste **Unités d’organisation** du déploiement. L’enregistrement de l’outil seul n’est pas suffisant ; le déploiement détermine quels cours voient l’outil.

**`deployment_id` ne correspond pas au lancement.** FastComments enregistre de manière TOFU le premier `deployment_id` qu’il voit pour une inscription. Si un administrateur supprime le déploiement initial et en crée un nouveau, les lancements depuis le nouveau déploiement sont rejetés avec une erreur de non-correspondance de déploiement. La solution consiste à réenregistrer FastComments (générer une nouvelle URL d’inscription et relancer l’enregistrement dynamique) ; l’ancien enregistrement de configuration est remplacé.

**L’outil se lance mais affiche « Invalid LTI launch ».** Le cours se trouve dans une structure de locataire/organisation différente de celle couverte par le déploiement, ou le déploiement a été désactivé après l’enregistrement. Revérifiez **Outils d’administration** > **Gérer l’extensibilité** > **LTI Advantage** > FastComments > bascule **Activé** et la liste des unités organisationnelles du déploiement.

**Noms et rôles manquants dans FastComments.** Brightspace envoie les lancements LTI avec les revendications NRPS (Names and Role Provisioning Services). Si un cours a été mis à niveau depuis un ancien lien LTI 1.1, le lancement peut manquer les revendications `name` et `email`. Réajoutez le sujet FastComments via **Ajouter un élément existant** (ne migrez pas l’ancien lien) afin que le lancement utilise LTI 1.3.

**L’intégration affiche un écran de connexion au lieu de l’authentification SSO automatique.** Le sujet HTML a été inséré comme une simple balise `<iframe>` pointant vers FastComments au lieu d’être inséré via **Insérer du contenu** > **LTI Advantage**. Les iframes simples contournent le lancement LTI et dirigent les utilisateurs vers la page publique de FastComments. Supprimez l’iframe et réinsérez-le via le flux Insérer du contenu.