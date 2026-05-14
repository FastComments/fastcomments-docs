Une fois qu’un administrateur a enregistré FastComments en tant qu’outil LTI 1.3 Advantage et approuvé les politiques de l’établissement, les instructeurs l’ajoutent aux cours via les points de placement standard de Blackboard. Les étapes exactes diffèrent entre Ultra Course View et Original Course View, donc les deux sont décrites ci‑dessous.

#### Ultra Course View

Ultra Course View est l’affichage par défaut dans Blackboard Learn SaaS à partir de 2026.

1. Ouvrez le cours et accédez à la page **Course Content**.
2. Survolez ou touchez l’endroit où vous voulez que le fil de commentaires apparaisse dans le plan et cliquez sur le bouton violet **+** (Add content).
3. Choisissez **Content Market**. Le panneau Content Market répertorie tous les outils LTI approuvés et les placements Building Block pour votre établissement.
4. Trouvez la vignette **FastComments** et cliquez dessus. Blackboard crée un élément de contenu à l’emplacement où vous avez ouvert le menu **+**.
5. Par défaut, l’élément apparaît dans le plan en tant qu’entrée « Visible to students » pour les instructeurs dont le paramètre personnel par défaut **Hide from students** est désactivé. Si votre paramètre par défaut est **Hidden**, l’élément est créé en mode masqué et vous activez le sélecteur de visibilité sur la ligne de l’élément lorsque vous êtes prêt.
6. Pour renommer l’élément, cliquez sur le titre dans le plan et saisissez une nouvelle étiquette. Le titre que voient les étudiants dans le plan est indépendant de l’identifiant du fil FastComments, donc le renommage est sans risque à tout moment.

Si vous ne voyez pas **Content Market** comme option, votre établissement a masqué ce placement. Vous pouvez aussi accéder au même sélecteur via **More tools** dans le même menu **+** sous le groupe **LTI Tools**.

#### Original Course View

Original Course View est toujours pris en charge dans Learn SaaS et reste l’expérience principale pour les sites Learn 9.1 auto‑hébergés sur la ligne de release Q4 2024 CU.

1. Ouvrez le cours et entrez dans une **Content Area** (par exemple, la zone par défaut **Information** ou **Content** dans le menu du cours).
2. Activez **Edit Mode** avec le bouton bascule dans le coin supérieur droit de la page.
3. Cliquez sur **Build Content** dans la barre d’actions.
4. Sous le sous‑menu **Learning Tools**, cliquez sur **FastComments**. Le sous‑menu Learning Tools est rempli à partir des placements d’outils LTI 1.3 après qu’un administrateur ait enregistré l’outil. Si vous ne le voyez pas, consultez la section des problèmes connus ci‑dessous.
5. Dans le formulaire **Create FastComments**, définissez :
   - **Name** : l’étiquette que voient les étudiants dans la zone de contenu.
   - **Description** : texte facultatif affiché au‑dessus du fil intégré.
   - **Permit Users to View this Content** : bascule de disponibilité Oui/Non.
   - **Track Number of Views** : activez si vous souhaitez les statistiques de consultation par élément de Blackboard. FastComments exécute ses propres analyses de manière indépendante.
   - **Date and Time Restrictions** : fenêtres optionnelles **Display After** / **Display Until**.
6. Soumettez. L’outil apparaît comme un élément cliquable dans la zone de contenu.

#### Intégration dans un élément ou un document

Dans les deux modes de cours, les instructeurs intègrent FastComments en ligne dans le corps d’un Item, d’un Document ou de tout champ riche via le bouton LTI Advantage de l’éditeur de contenu.

Ultra Course View :

1. Créez ou modifiez un **Document**.
2. Cliquez sur **Add content** à l’intérieur du corps du document à l’endroit où vous voulez que le fil apparaisse.
3. Dans la barre d’outils de l’éditeur, ouvrez le menu **Insert content** et cliquez sur **Content Market** (le point d’entrée LTI Advantage / Deep Linking).
4. Sélectionnez **FastComments**. FastComments renvoie une charge utile deep‑link et Blackboard insère un bloc intégré dans le corps du document à la position du curseur.
5. Enregistrez le document. Les étudiants voient le fil affiché en ligne lorsqu’ils font défiler la page.

Original Course View :

1. Modifiez n’importe quel élément ayant un corps en texte riche.
2. Dans la barre d’outils de l’éditeur de contenu, cliquez sur l’icône plus **Add Content** et choisissez **Content Market** (libellé **Add Content from External Tool** dans les anciens CU de Q4 2024).
3. Sélectionnez **FastComments**. L’éditeur insère un bloc de remplacement référant la ressource deep‑linkée.
4. Soumettez l’élément.

Chaque intégration deep‑link crée son propre fil FastComments, donc un Item avec deux blocs FastComments intégrés possède deux flux de commentaires indépendants.

#### Visibilité, conditions de publication et restrictions de groupe

Les éléments de contenu FastComments se comportent comme tout autre élément de contenu Blackboard pour ce qui est des règles de contrôle d’accès qui leur sont appliquées.

- Ultra : cliquez sur le sélecteur de visibilité sur la ligne (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilité conditionnelle prend en charge les fenêtres de date/heure, les règles de performance par rapport aux éléments du carnet de notes et les règles de membre liées aux groupes de cours.
- Original : ouvrez le menu contextuel de l’élément et choisissez **Adaptive Release** ou **Adaptive Release: Advanced** pour restreindre l’accès à l’outil par date, appartenance, note ou statut de révision. Utilisez **Set Group Availability** sur l’élément pour le restreindre à des groupes de cours spécifiques.

FastComments respecte la décision de verrouillage prise par Blackboard. Si Blackboard masque l’élément pour un étudiant, le lancement LTI ne s’exécute jamais pour cet étudiant et il n’apparaît pas dans la vue modérateur.

#### Comportement dans le carnet de notes

FastComments ne renvoie pas de notes via LTI Advantage Assignment and Grade Services. Aucune colonne de notes n’est créée automatiquement pour les éléments de contenu FastComments.

Si votre locataire Blackboard est configuré pour créer automatiquement une colonne du carnet de notes pour chaque nouvel élément de contenu indépendamment des métadonnées de notation, une colonne vide apparaît quand même. Pour la masquer :

- Ultra : ouvrez le **Gradebook**, cliquez sur l’en‑tête de colonne, choisissez **Edit**, et désactivez **Show to students** ainsi que **Include in calculations**. Ou utilisez **Delete** si votre établissement permet la suppression de colonne pour les éléments non notés.
- Original : ouvrez le **Grade Center**, cliquez sur le chevron de la colonne, choisissez **Hide from Users (on/off)**, et éventuellement **Hide from Instructor View** sous **Column Organization**.

#### Ce que voient les étudiants

Lorsque l’étudiant ouvre l’élément FastComments ou fait défiler jusqu’à un bloc intégré :

1. Blackboard lance le message LTI 1.3 vers FastComments. L’étudiant est connecté via SSO en utilisant son identité Blackboard (nom, courriel, avatar, rôle) sans voir de formulaire de connexion.
2. Le fil de commentaires s’affiche dans l’iframe. Le fil, les réponses, les mentions et les réactions sont tous disponibles en fonction des paramètres du widget de commentaires configurés dans FastComments.
3. Leurs commentaires sont attribués à leur compte Blackboard. Si l’étudiant modifie son nom ou sa photo dans Blackboard ultérieurement, le prochain lancement mettra à jour le profil FastComments.

Mappage des rôles de Blackboard vers FastComments :

- **System Administrator** et **Course Builder** correspondent à FastComments **admin**.
- **Instructor** et **Teaching Assistant** correspondent à FastComments **moderator**.
- **Student**, **Guest**, et **Observer** correspondent à FastComments **commenter**.

Les modérateurs voient les contrôles de modération (pin, hide, ban, delete) intégrés à chaque commentaire du fil.

#### Portée des fils

FastComments scope chaque fil par **(Blackboard host, course ID, resource link ID)**. Deux éléments FastComments dans le même cours produisent deux fils. Le même élément copié dans deux shells de cours (par exemple, via la copie de cours) produit deux fils, car Blackboard émet un nouvel resource link ID lors de la copie. Pour conserver un fil partagé entre des copies de cours, utilisez Deep Linking avec un URN de fil explicite configuré dans FastComments avant d’exécuter la copie.

#### Problèmes spécifiques à Blackboard

**La vignette FastComments manque dans le menu Build Content (Original) ou Content Market (Ultra).** L’administrateur a approuvé l’outil mais a laissé une politique d’établissement bloquant le placement pertinent. Allez dans **Administrator Panel** > **Integrations** > **LTI Tool Providers**, modifiez l’entrée FastComments, et confirmez que les placements **Course Content Tool** (Original) et **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) sont activés. Enregistrez et actualisez la page du cours.

**Erreur "Tool not configured for this context" ou "Tool is not deployed" au lancement.** La portée de déploiement enregistrée lors de l’enregistrement dynamique ne correspond pas au contexte d’établissement auquel le cours appartient. Dans l’entrée du provider d’outil de Blackboard, vérifiez que le **Deployment ID** correspond à ce que FastComments affiche sur sa page de configuration LTI 1.3 pour ce locataire. S’ils diffèrent, supprimez le placement et relancez l’enregistrement dynamique à partir d’une URL d’enregistrement fraîche.

**La hauteur de l’iframe semble fixe ou le contenu est tronqué.** Certains locataires Blackboard disposent d’une politique de sécurité de contenu stricte qui bloque le postMessage de redimensionnement d’iframe LTI par défaut. FastComments émet à la fois le message de style Canvas `lti.frameResize` et le message de la spécification IMS `org.imsglobal.lti.frameResize` pour maximiser la compatibilité, mais une substitution CSP au niveau du locataire peut bloquer l’écouteur parent. Demandez à votre administrateur de confirmer que `*.fastcomments.com` est sur la liste blanche des outils LTI et qu’aucun en‑tête CSP personnalisé ne supprime les événements postMessage. Le redimensionnement fonctionnera alors sans configuration supplémentaire.

**La copie de cours duplique les fils.** La copie de cours Blackboard génère de nouveaux resource link ID pour les placements LTI, donc les cours copiés démarrent avec des fils vides. C’est normal. Si vous avez besoin que le cours copié hérite du fil d’origine, configurez Deep Linking avec un URN de fil explicite avant la copie, ou contactez le support FastComments pour remapper les identifiants de fil en masse.

**L’étudiant voit une erreur générique Blackboard au lancement.** La cause est une revendication `email` manquante ou périmée. Confirmez que la politique de l’établissement pour FastComments a **Role**, **Name**, et **Email Address** activés sous **User Fields to Send**. Enregistrez, puis relancez dans une nouvelle session de navigateur.