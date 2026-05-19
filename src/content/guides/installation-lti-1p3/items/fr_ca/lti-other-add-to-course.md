Une fois que FastComments est enregistré sur la plateforme, les instructeurs l'ajoutent au contenu du cours en utilisant les flux standard d'outils externes de la plateforme. Cette page couvre Sakai 23.x et Schoology Enterprise.

#### Verrouiller l'accès public (Recommandé)

Par défaut, les données de commentaires FastComments sont lisibles publiquement sur chaque plateforme. Toute personne capable de deviner l'URL d'un fil ou le point de terminaison API peut voir ses commentaires, même en dehors de Sakai ou Schoology. Pour les discussions de cours, vous voudrez presque certainement restreindre l'affichage aux étudiants inscrits seulement.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Require SSO To View Comments** activé, puis définissez le niveau de sécurité sur **Secure SSO** afin que les fils puissent être chargés uniquement via le lancement LTI signé.

Voir [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou une seule page.

#### Sakai

**1. Ajouter FastComments à un site**

Le gestionnaire du site active l'outil au niveau de chaque site :

1. Ouvrez le site et cliquez sur **Site Info** dans la navigation de gauche.
2. Cliquez sur **Manage Tools**.
3. Faites défiler jusqu'à la liste **External Tools** et activez **FastComments**.
4. Cliquez sur **Continue**, vérifiez la liste des outils, puis cliquez sur **Finish**.

FastComments apparaît maintenant comme un élément de la navigation de gauche dans le site.

**2. Réordonner l'entrée de navigation de gauche**

Allez dans **Site Info** > **Tool Order**. Faites glisser **FastComments** à la position souhaitée et cliquez sur **Save**. Vous pouvez aussi renommer le libellé de navigation et le masquer aux étudiants depuis cet écran.

**3. Intégrer en ligne dans une page Lessons**

Pour placer FastComments directement dans une page Lessons plutôt que comme un outil autonome dans la navigation de gauche :

1. Ouvrez l'outil **Lessons** dans le site.
2. Cliquez sur **Add Content** > **Add External Tool**.
3. Sélectionnez **FastComments** dans la liste.
4. Si FastComments a annoncé Deep Linking lors de l'enregistrement, Sakai ouvre le sélecteur de contenu de l'outil pour que vous puissiez choisir ou étiqueter le fil. Si Deep Linking n'a pas été annoncé, Sakai insère un lien de lancement par défaut.
5. Enregistrez l'élément Lessons.

Chaque instance intégrée obtient son propre fil, limité à ce lien de ressource.

**4. Ajustements de permission pour l'accès des étudiants**

Sakai contrôle les lancements d'outils externes via les Realms. Pour confirmer que les étudiants peuvent lancer FastComments :

1. Connectez-vous en tant qu'administrateur Sakai et ouvrez **Administration Workspace** > **Realms**.
2. Ouvrez le realm concerné (par exemple, `!site.template.course` ou le realm du site spécifique).
3. Confirmez que le rôle `access` a `lti.launch` activé et que les permissions de rôle dans le groupe **external.tools** sont accordées.
4. Enregistrez le realm.

Pour les substitutions au niveau du site, le gestionnaire peut ajuster la visibilité de l'outil par rôle depuis **Site Info** > **Tool Order** en masquant ou en affichant FastComments par rôle.

**5. Ce que voient les étudiants**

Les étudiants cliquent sur l'élément FastComments de la navigation de gauche (ou font défiler jusqu'au bloc Lessons intégré) et arrivent directement dans la vue des fils de discussion. Le SSO est automatique : Sakai envoie l'identité de l'utilisateur dans le lancement LTI et FastComments les connecte sous leur compte Sakai.

Mappage des rôles :

- Sakai `Instructor` -> modérateur FastComments
- Sakai `Admin` (admin in Administration Workspace) -> administrateur FastComments
- Sakai `Student` / `access` -> commentateur FastComments

**6. Pièges Sakai**

- **Outil non visible dans Manage Tools.** Si FastComments n'apparaît pas dans la liste External Tools, l'administrateur Sakai doit ouvrir le registre d'outils (**Administration Workspace** > **External Tools** > **FastComments**) et définir **Stealthed** sur `false`. Les outils stealthed sont cachés du sélecteur Manage Tools au niveau du site.
- **Lancements qui échouent dans des navigateurs en session partagée.** Le jeton CSRF du portail Sakai est lié à la session du navigateur. Si un étudiant est connecté à deux sites Sakai dans différents onglets ou possède une session périmée, le lancement retourne un 403. Solution : fermez les autres onglets Sakai, déconnectez-vous, reconnectez-vous et relancez. Les administrateurs peuvent aussi augmenter `sakai.csrf.token.cache.ttl` si cela se produit à l'échelle du cluster.
- **Intégration en iframe.** Confirmez que `lti.frameheight` dans `sakai.properties` est suffisamment grand (600 ou plus) afin que le fil de commentaires ne soit pas coupé dans une page Lessons.

#### Schoology

Schoology Enterprise a deux scénarios d'installation. Confirmez lequel s'applique avant d'ajouter l'outil à un cours.

**1. Deux scénarios d'installation**

- **(a) Installation au niveau de l'entreprise.** L'administrateur système Schoology a installé FastComments au niveau de l'organisation et l'a attribué à tous les cours ou à des modèles de cours spécifiques. Les instructeurs sautent l'installation et passent directement à « Add Materials ».
- **(b) Installation par l'instructeur.** L'instructeur installe l'outil dans un seul cours via **Course Options** > **External Tools** > **Install LTI Apps**. L'auto-installation nécessite que l'administrateur système ait approuvé l'application FastComments au niveau de l'organisation au préalable.

**2. Ajouter FastComments en tant que matériel de cours**

À l'intérieur du cours :

1. Ouvrez le cours et allez dans **Materials**.
2. Cliquez sur **Add Materials** > **Add File/Link/External Tool**.
3. Choisissez **External Tool**.
4. Sélectionnez **FastComments** dans la liste des outils enregistrés.
5. Définissez un **Name** (c'est ce que les étudiants voient dans la liste des matériels) et une **Description** facultative.
6. Laissez **Enable Grading** (grade passback) **OFF**. FastComments ne renvoie pas de notes à Schoology, donc activer le grade passback crée une colonne vide dans le carnet de notes.
7. Cliquez sur **Submit**.

Le matériel apparaît maintenant dans la liste des matériels du cours et ouvre le fil FastComments lorsqu'on clique dessus.

**3. Intégration en ligne via l'éditeur Rich Text**

Si l'administrateur système a activé le placement Deep Linking pour FastComments lors de l'enregistrement, les instructeurs peuvent intégrer le fil de commentaires dans n'importe quel champ Rich Text (instructions de devoir, corps de page, consignes de discussion) :

1. Ouvrez l'éditeur Rich Text sur la page cible.
2. Cliquez sur l'icône **External Tool** (pièce de puzzle) dans la barre d'outils.
3. Choisissez **FastComments**.
4. Configurez l'intégration dans la boîte de dialogue de deep-linking et cliquez sur **Insert**.
5. Enregistrez la page.

Si le bouton External Tool n'apparaît pas dans l'éditeur Rich Text, Deep Linking est désactivé pour cet outil sur ce tenant. Voir les pièges ci-dessous.

**4. Visibilité et affectations par section**

Schoology limite la disponibilité des outils par section via Course Options :

1. Depuis le cours, cliquez sur **Course Options** > **External Tools**.
2. Pour chaque application LTI installée, vous contrôlez si elle est disponible pour toutes les sections du cours ou pour des sections spécifiques.
3. Pour restreindre FastComments à certaines sections, décochez les sections qui ne doivent pas voir l'outil.
4. L'accès au niveau de la section conditionne également quelles sections voient l'entrée **Add Materials** > **External Tool** pour FastComments.

**5. Ce que voient les étudiants**

Les étudiants cliquent sur le matériel FastComments (ou font défiler jusqu'à l'intégration en ligne) et arrivent dans la discussion filée. Le SSO est automatique via le lancement LTI Schoology sous leur compte Schoology.

Mappage des rôles :

- Schoology `Administrator` -> administrateur FastComments
- Schoology `Instructor` -> modérateur FastComments
- Schoology `Student` -> commentateur FastComments

**6. Pièges Schoology**

- **Uniquement Enterprise.** Les comptes Schoology personnels et gratuits ne peuvent pas installer d'outils LTI 1.3. Si votre tenant est sur le forfait gratuit, l'option **External Tools** est absente de Course Options. Passez à Schoology Enterprise pour utiliser FastComments.
- **Deep Linking désactivé par défaut au niveau du tenant.** Certains tenants Schoology restreignent le placement Deep Linking au niveau de l'organisation. Quand c'est le cas, les instructeurs ne voient que le flux **Add Materials** > **External Tool** et pas le bouton External Tool dans l'éditeur Rich Text. Pour permettre l'intégration en ligne, l'administrateur système doit aller dans **System Settings** > **Integration** > **LTI 1.3** > **FastComments** et activer le placement **Content Item / Deep Linking**, puis enregistrer.
- **Substitution d'affectation par section.** Si FastComments est assigné au niveau de l'entreprise mais que l'instructeur ne le voit pas dans **Add Materials**, la section du cours est exclue de l'affectation au niveau organisationnel. Demandez à l'administrateur système d'ajouter la section à l'affectation de l'application FastComments.
- **Nom du matériel vs identité du fil.** Renommer le matériel dans Schoology ne déplace pas le fil de commentaires. Les fils sont identifiés par l'ID du lien de ressource LTI, donc un renommage conserve le même fil ; supprimer et recréer le matériel crée un nouveau fil vide.