Une fois que FastComments est enregistré sur la plateforme, les enseignants l'ajoutent au contenu du cours en utilisant les flux standard d'outil externe de la plateforme. Cette page couvre Sakai 23.x et Schoology Enterprise.

#### Restreindre l'accès public (recommandé)

Par défaut, les données de commentaires FastComments sont lisibles publiquement sur les deux plateformes. Toute personne pouvant deviner l'URL d'un fil ou un point de terminaison API peut voir ses commentaires, même en dehors de Sakai ou Schoology. Pour les discussions de cours, vous voudrez presque certainement restreindre la consultation aux seuls étudiants inscrits.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Exiger SSO pour voir les commentaires** activé, puis réglez le niveau de sécurité sur **SSO sécurisé** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Voir [Protéger les fils de commentaires avec l'authentification unique](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou à une seule page.

#### Sakai

**1. Ajouter FastComments à un site**

Le gestionnaire du site active l'outil par site :

1. Ouvrez le site et cliquez sur **Site Info** dans la navigation de gauche.
2. Cliquez sur **Manage Tools**.
3. Faites défiler jusqu'à la liste **External Tools** et activez **FastComments**.
4. Cliquez sur **Continue**, vérifiez la liste des outils, puis cliquez sur **Finish**.

FastComments apparaît maintenant comme un élément de navigation à gauche dans le site.

**2. Réorganiser l'entrée de la navigation à gauche**

Allez dans **Site Info** > **Tool Order**. Faites glisser **FastComments** à l'emplacement souhaité et cliquez sur **Save**. Vous pouvez également renommer le libellé de navigation et le masquer aux étudiants depuis cet écran.

**3. Intégrer en ligne dans une page Lessons**

Pour placer FastComments directement à l'intérieur d'une page Lessons plutôt que comme un outil autonome dans la navigation à gauche :

1. Ouvrez l'outil **Lessons** dans le site.
2. Cliquez sur **Add Content** > **Add External Tool**.
3. Sélectionnez **FastComments** dans la liste.
4. Si FastComments a annoncé Deep Linking lors de l'enregistrement, Sakai ouvre le sélecteur de contenu de l'outil afin que vous puissiez choisir ou étiqueter le fil. Si Deep Linking n'a pas été annoncé, Sakai insère un lien de lancement par défaut.
5. Enregistrez l'élément Lessons.

Chaque instance intégrée obtient son propre fil, limité à ce lien de ressource.

**4. Ajustements de permissions pour l'accès étudiant**

Sakai gère les lancements d'outils externes via les Realms. Pour confirmer que les étudiants peuvent lancer FastComments :

1. Connectez-vous en tant qu'administrateur Sakai et ouvrez **Administration Workspace** > **Realms**.
2. Ouvrez le realm concerné (par exemple, `!site.template.course` ou le realm spécifique du site).
3. Confirmez que le rôle `access` a `lti.launch` activé et que les permissions de rôle dans le groupe **external.tools** sont accordées.
4. Enregistrez le realm.

Pour les remplacements au niveau du site, le mainteneur peut ajuster la visibilité de l'outil par rôle depuis **Site Info** > **Tool Order** en masquant ou en affichant FastComments par rôle.

**5. Ce que voient les étudiants**

Les étudiants cliquent sur l'élément FastComments dans la navigation à gauche (ou font défiler jusqu'au bloc Lessons intégré) et arrivent directement dans la vue de fil de commentaires. Le SSO est automatique : Sakai envoie l'identité de l'utilisateur dans le lancement LTI et FastComments le connecte sous son compte Sakai.

Mappage des rôles :

- Sakai `Instructor` -> modérateur FastComments
- Sakai `Admin` (admin dans Administration Workspace) -> administrateur FastComments
- Sakai `Student` / `access` -> commentateur FastComments

**6. Points sensibles Sakai**

- **Outil non visible dans Manage Tools.** Si FastComments n'apparaît pas dans la liste External Tools, l'administrateur Sakai doit ouvrir le registre des outils (**Administration Workspace** > **External Tools** > **FastComments**) et régler **Stealthed** sur `false`. Les outils en mode stealth sont cachés du sélecteur Manage Tools par site.
- **Lancements cassés dans les navigateurs à session partagée.** Le jeton CSRF du portail Sakai est lié à la session du navigateur. Si un étudiant est connecté à deux sites Sakai dans des onglets différents ou a une session obsolète, le lancement renvoie un 403. Correction : fermez les autres onglets Sakai, déconnectez-vous, reconnectez-vous et relancez. Les administrateurs peuvent aussi augmenter `sakai.csrf.token.cache.ttl` si cela se produit à l'échelle du cluster.
- **Intégration dans une iframe.** Vérifiez que `lti.frameheight` dans `sakai.properties` est suffisamment grand (600 ou plus) afin que le fil de commentaires ne soit pas coupé à l'intérieur d'une page Lessons.

#### Schoology

Schoology Enterprise a deux scénarios d'installation. Confirmez lequel s'applique avant d'ajouter l'outil à un cours.

**1. Deux scénarios d'installation**

- **(a) Installation au niveau de l'entreprise.** L'administrateur système Schoology a installé FastComments au niveau de l'organisation et l'a attribué à tous les cours ou à des modèles de cours spécifiques. Les enseignants n'ont pas besoin d'installer l'outil et passent directement à « Ajouter du contenu ».
- **(b) Auto-installation par l'enseignant.** L'enseignant installe l'outil dans un cours unique depuis **Course Options** > **External Tools** > **Install LTI Apps**. L'auto-installation nécessite que l'administrateur système ait d'abord approuvé l'application FastComments au niveau de l'organisation.

**2. Ajouter FastComments comme matériel de cours**

Dans le cours :

1. Ouvrez le cours et allez dans **Materials**.
2. Cliquez sur **Add Materials** > **Add File/Link/External Tool**.
3. Choisissez **External Tool**.
4. Sélectionnez **FastComments** dans la liste des outils enregistrés.
5. Définissez un **Name** (c'est ce que voient les étudiants dans la liste des matériels) et une **Description** optionnelle.
6. Laissez **Enable Grading** (rapprochement des notes) **OFF**. FastComments ne renvoie pas de notes à Schoology, donc l'activation du rapprochement crée une colonne vide dans le carnet de notes.
7. Cliquez sur **Submit**.

Le matériel apparaît désormais dans la liste des matériels du cours et ouvre le fil FastComments lorsqu'on clique dessus.

**3. Intégration en ligne via l'éditeur Rich Text**

Si l'administrateur système a activé le placement Deep Linking pour FastComments lors de l'enregistrement, les enseignants peuvent intégrer le fil de commentaires dans n'importe quel champ Rich Text (instructions de devoir, corps des pages, consignes de discussion) :

1. Ouvrez l'éditeur Rich Text sur la page cible.
2. Cliquez sur l'icône **External Tool** (pièce de puzzle) dans la barre d'outils.
3. Choisissez **FastComments**.
4. Configurez l'intégration dans la boîte de dialogue de deep-linking et cliquez sur **Insert**.
5. Enregistrez la page.

Si le bouton External Tool n'apparaît pas dans l'éditeur Rich Text, Deep Linking est désactivé pour cet outil sur ce tenant. Voir les points sensibles ci-dessous.

**4. Visibilité et affectation par section**

Schoology gère la disponibilité des outils par section via Course Options :

1. Depuis le cours, cliquez sur **Course Options** > **External Tools**.
2. Pour chaque application LTI installée, vous contrôlez si elle est disponible pour toutes les sections du cours ou pour des sections spécifiques.
3. Pour restreindre FastComments à certaines sections, décochez les sections qui ne doivent pas voir l'outil.
4. L'accès au niveau de la section détermine également quelles sections voient l'entrée **Add Materials** > **External Tool** pour FastComments.

**5. Ce que voient les étudiants**

Les étudiants cliquent sur le matériel FastComments (ou font défiler jusqu'à l'intégration en ligne) et atterrissent dans la discussion filée. Le SSO est automatique via le lancement LTI Schoology sous leur compte Schoology.

Mappage des rôles :

- Schoology `Administrator` -> administrateur FastComments
- Schoology `Instructor` -> modérateur FastComments
- Schoology `Student` -> commentateur FastComments

**6. Points sensibles Schoology**

- **Réservé à l'entreprise.** Les comptes Schoology personnels et gratuits ne peuvent pas installer d'outils LTI 1.3. Si votre tenant est sur le forfait gratuit, l'option **External Tools** est absente de Course Options. Passez à Schoology Enterprise pour utiliser FastComments.
- **Deep Linking désactivé par défaut par le tenant.** Certains tenants Schoology restreignent le placement Deep Linking au niveau de l'organisation. Dans ce cas, les enseignants ne voient que le flux **Add Materials** > **External Tool** et pas le bouton External Tool dans l'éditeur Rich Text. Pour permettre l'intégration en ligne, l'administrateur système doit aller dans **System Settings** > **Integration** > **LTI 1.3** > **FastComments** et activer le placement **Content Item / Deep Linking**, puis enregistrer.
- **Remplacement par affectation par section.** Si FastComments est attribué au niveau de l'entreprise mais que l'enseignant ne peut pas le voir dans **Add Materials**, la section du cours est exclue dans l'affectation au niveau de l'organisation. Demandez à l'administrateur système d'ajouter la section à l'affectation de l'application FastComments.
- **Nom du matériel vs identité du fil.** Renommer le matériel dans Schoology ne déplace pas le fil de commentaires. Les fils sont indexés par l'ID du lien de ressource LTI, donc un renommage conserve le même fil ; supprimer puis recréer le matériel crée un nouveau fil vide.