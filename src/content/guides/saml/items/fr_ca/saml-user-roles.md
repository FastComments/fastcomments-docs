FastComments mappe les rôles utilisateur SAML aux autorisations internes, permettant le contrôle d'accès basé sur les rôles pour votre organisation.

### Système de rôles FastComments

FastComments utilise un système d'autorisations basé sur les rôles où les utilisateurs peuvent avoir un ou plusieurs rôles qui déterminent leurs niveaux d'accès et leurs capacités.

### Rôles FastComments disponibles

#### Rôles administratifs

**fc-account-owner**
- **Autorisations** : Accès administratif complet
- **Capacités** : Toutes les fonctionnalités, gestion de la facturation, gestion des utilisateurs
- **Cas d'utilisation** : Administrateurs principaux du compte et propriétaires

**fc-admin-admin**  
- **Autorisations** : Accès administratif à la plupart des fonctionnalités
- **Capacités** : Gestion des utilisateurs, configuration, modération. **Peut administrer d'autres administrateurs.**
- **Cas d'utilisation** : Administrateurs secondaires et personnel informatique

**fc-billing-admin**
- **Autorisations** : Gestion de la facturation et des abonnements
- **Capacités** : Méthodes de paiement, factures, modifications d'abonnement
- **Cas d'utilisation** : Membres de l'équipe financière et contacts pour la facturation

#### Rôles spécialisés

**fc-analytics-admin**
- **Autorisations** : Accès aux analyses et rapports
- **Capacités** : Voir les statistiques du site, les données d'engagement des utilisateurs
- **Cas d'utilisation** : Équipes marketing et analystes de données

**fc-api-admin**
- **Autorisations** : Accès et gestion de l'API
- **Capacités** : Identifiants API, configuration des webhooks
- **Cas d'utilisation** : Développeurs et intégrateurs techniques

**fc-moderator**
- **Autorisations** : Capacités de modération des commentaires
- **Capacités** : Approuver/rejeter des commentaires, gérer le spam
- **Cas d'utilisation** : Modérateurs de communauté et gestionnaires de contenu

### Configuration du mappage des rôles

#### Sources d'attributs SAML

FastComments accepte les informations de rôle provenant de différents noms d'attributs SAML pour assurer la compatibilité avec différents fournisseurs d'identité :

**Noms d'attributs standard** :
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Attributs Microsoft/ADFS** :
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Formats de rôle pris en charge

**Format tableau** *(Préféré)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format séparé par des virgules**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Format à rôle unique**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Configuration des rôles côté fournisseur d'identité

#### Microsoft Azure AD

1. **Configuration des App Roles** :
   - Définissez les rôles FastComments dans votre application Azure AD
   - Assignez les utilisateurs aux app roles appropriés
   - Configurez les claims pour inclure les rôles assignés

2. **Mappage d'attributs** :
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Attribution de groupes** :
   - Créez des groupes correspondant aux noms de rôles FastComments
   - Assignez les utilisateurs aux groupes appropriés
   - Configurez les déclarations d'attributs

2. **Déclaration d'attribut** :
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mappage de groupes** :
   - Créez des unités organisationnelles ou des groupes
   - Nommez les groupes avec les préfixes de rôle FastComments
   - Configurez le mappage d'attributs

2. **Attributs personnalisés** :
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Comportement utilisateur par défaut

#### Utilisateurs sans rôles

Lorsqu'un utilisateur SAML n'a aucun rôle ou a des rôles non reconnus :
- L'utilisateur est créé en tant que commentateur standard
- Aucun accès administratif n'est accordé
- Peut publier et gérer ses propres commentaires
- Ne peut pas accéder aux fonctionnalités du tableau de bord d'administration

#### Héritage des rôles

- Les utilisateurs peuvent avoir plusieurs rôles simultanément
- Les autorisations sont cumulatives (le niveau d'autorisation le plus élevé s'applique)
- Les modifications de rôles dans l'IdP sont reflétées à la prochaine connexion

### Gestion des utilisateurs SAML

#### Création d'un utilisateur

Lorsqu'un utilisateur se connecte via SAML pour la première fois :
1. **Compte utilisateur** : Créé automatiquement avec l'email comme identifiant
2. **Attribution des rôles** : Les rôles sont appliqués en fonction des attributs SAML
3. **Informations de profil** : Prénom/nom renseignés si fournis
4. **Activation des autorisations** : Les rôles deviennent actifs immédiatement

#### Mises à jour des rôles

Les utilisateurs SAML existants reçoivent des mises à jour de rôles :
1. **Déclencheur de connexion** : Les mises à jour de rôle ont lieu à chaque connexion SAML
2. **Effet immédiat** : Les nouvelles autorisations s'appliquent immédiatement
3. **Suppression de rôle** : Les rôles supprimés sont révoqués automatiquement
4. **Piste d'audit** : Les changements de rôles sont consignés dans les logs d'audit

### Mappage de rôles personnalisé

#### Personnalisation pour les entreprises

Pour les clients entreprise ayant des exigences spécifiques :
- Des noms de rôles personnalisés peuvent être mappés aux autorisations FastComments
- Des hiérarchies de rôles complexes peuvent être mises en place
- Des contrôles d'accès spécifiques par département peuvent être configurés

Contactez le support FastComments pour les configurations de mappage de rôles personnalisés.

#### Validation des rôles

FastComments valide les rôles entrants :
- Les rôles non reconnus sont ignorés (non rejetés)
- Les attributs de rôle mal formés sont consignés pour le dépannage
- Les utilisateurs conservent leurs rôles existants si l'assertion SAML ne contient pas d'information de rôle

### Bonnes pratiques

#### Gestion des rôles

1. **Principe du moindre privilège** : Attribuez les autorisations minimales nécessaires
2. **Audits réguliers** : Passez en revue les rôles et accès des utilisateurs périodiquement  
3. **Nommage clair** : Utilisez des noms de groupes descriptifs dans votre IdP
4. **Documentation** : Tenez à jour la documentation des attributions de rôles

#### Considérations de sécurité

1. **Attributs de rôle** : Assurez-vous que les attributs de rôle sont correctement sécurisés dans les réponses SAML
2. **Validation des attributs** : Vérifiez que seuls les systèmes autorisés peuvent assigner des rôles
3. **Revue des accès** : Passez régulièrement en revue les attributions de rôles administratifs
4. **Surveillance** : Surveillez les changements de rôles et les actions administratives

### Dépannage des problèmes de rôle

#### Problèmes courants

**Rôles non appliqués** :
- Vérifiez que les noms d'attributs SAML correspondent aux formats pris en charge
- Vérifiez que l'IdP envoie les informations de rôle
- Confirmez que les valeurs de rôle correspondent exactement aux noms de rôles FastComments

**Accès refusé** :
- Vérifiez que l'utilisateur a le rôle approprié attribué dans l'IdP
- Vérifiez l'orthographe des rôles et la sensibilité à la casse
- Confirmez que le rôle est correctement formaté dans la réponse SAML

**Autorisations manquantes** :
- Passez en revue les définitions de rôles et les autorisations requises
- Vérifiez les attributions de rôles conflictuelles
- Vérifiez que l'utilisateur s'est connecté après les modifications de rôle

---