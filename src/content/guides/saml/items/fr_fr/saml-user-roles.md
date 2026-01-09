FastComments cartographie les rôles d'utilisateur SAML aux autorisations internes, permettant le contrôle d'accès basé sur les rôles pour votre organisation.

### Système de rôles FastComments

FastComments utilise un système d'autorisations basé sur les rôles où les utilisateurs peuvent avoir un ou plusieurs rôles déterminant leurs niveaux d'accès et leurs capacités.

### Rôles FastComments disponibles

#### Rôles administratifs

**fc-account-owner**
- **Permissions**: Accès administratif complet
- **Capabilities**: Toutes les fonctionnalités, gestion de la facturation, gestion des utilisateurs
- **Use Case**: Administrateurs principaux du compte et propriétaires

**fc-admin-admin**  
- **Permissions**: Accès administratif à la plupart des fonctionnalités
- **Capabilities**: Gestion des utilisateurs, configuration, modération. **Peut administrer d'autres administrateurs.**
- **Use Case**: Administrateurs secondaires et personnel informatique

**fc-billing-admin**
- **Permissions**: Gestion de la facturation et des abonnements
- **Capabilities**: Méthodes de paiement, factures, modifications d'abonnement
- **Use Case**: Membres de l'équipe financière et contacts de facturation

#### Rôles spécialisés

**fc-analytics-admin**
- **Permissions**: Accès aux analyses et rapports
- **Capabilities**: Voir les statistiques du site, les données d'engagement des utilisateurs
- **Use Case**: Équipes marketing et analystes de données

**fc-api-admin**
- **Permissions**: Accès et gestion de l'API
- **Capabilities**: Identifiants API, configuration des webhooks
- **Use Case**: Développeurs et intégrateurs techniques

**fc-moderator**
- **Permissions**: Capacités de modération des commentaires
- **Capabilities**: Approuver/rejeter les commentaires, gérer le spam
- **Use Case**: Modérateurs de la communauté et gestionnaires de contenu

### Configuration du mappage des rôles

#### Sources d'attributs SAML

FastComments accepte les informations de rôle provenant de différents noms d'attributs SAML afin d'assurer la compatibilité avec différents fournisseurs d'identité :

**Noms d'attributs standard**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Attributs Microsoft/ADFS**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Prise en charge des formats de rôle

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Configuration des rôles du fournisseur d'identité

#### Microsoft Azure AD

1. **Configuration des rôles d'application**:
   - Définissez les rôles FastComments dans votre application Azure AD
   - Assignez les utilisateurs aux rôles d'application appropriés
   - Configurez les claims pour inclure les rôles attribués

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Affectation de groupes**:
   - Créez des groupes correspondant aux noms de rôle FastComments
   - Assignez les utilisateurs aux groupes appropriés
   - Configurez les déclarations d'attributs

2. **Déclaration d'attribut**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Mappage de groupes**:
   - Créez des unités organisationnelles ou des groupes
   - Nommez les groupes avec les préfixes de rôle FastComments
   - Configurez le mappage des attributs

2. **Attributs personnalisés**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Comportement utilisateur par défaut

#### Utilisateurs sans rôles

Lorsqu'un utilisateur SAML n'a pas de rôles ou que les rôles ne sont pas reconnus :
- L'utilisateur est créé en tant que commentateur standard
- Aucun accès administratif n'est accordé
- Peut poster et gérer ses propres commentaires
- Ne peut pas accéder aux fonctionnalités du tableau de bord administrateur

#### Héritage des rôles

- Les utilisateurs peuvent avoir plusieurs rôles simultanément
- Les autorisations sont cumulatives (le niveau d'autorisation le plus élevé s'applique)
- Les modifications de rôle dans l'IdP sont reflétées lors de la prochaine connexion

### Gestion des utilisateurs SAML

#### Création d'utilisateur

Lorsque un utilisateur se connecte via SAML pour la première fois :
1. **Compte utilisateur** : Créé automatiquement avec l'adresse e-mail comme identifiant
2. **Attribution des rôles** : Les rôles sont appliqués en fonction des attributs SAML
3. **Informations de profil** : Prénom/nom renseignés si fournis
4. **Activation des autorisations** : Les rôles deviennent actifs immédiatement

#### Mises à jour des rôles

Les utilisateurs SAML existants reçoivent des mises à jour de rôle :
1. **Déclencheur de connexion** : Les mises à jour de rôle ont lieu à chaque connexion SAML
2. **Effet immédiat** : Les nouvelles autorisations s'appliquent immédiatement
3. **Suppression de rôle** : Les rôles supprimés sont révoqués automatiquement
4. **Piste d'audit** : Les modifications de rôle sont enregistrées dans les journaux d'audit

### Mappage de rôles personnalisé

#### Personnalisation pour les entreprises

Pour les clients d'entreprise ayant des exigences spécifiques :
- Des noms de rôles personnalisés peuvent être mappés aux autorisations FastComments
- Des hiérarchies de rôles complexes peuvent être implémentées
- Des contrôles d'accès spécifiques aux départements peuvent être configurés

Contactez le support FastComments pour des configurations de mappage de rôles personnalisées.

#### Validation des rôles

FastComments valide les rôles entrants :
- Les rôles non reconnus sont ignorés (non rejetés)
- Les attributs de rôle mal formés sont consignés pour le dépannage
- Les utilisateurs conservent leurs rôles existants si l'assertion SAML ne contient pas d'informations sur les rôles

### Bonnes pratiques

#### Gestion des rôles

1. **Principe du moindre privilège** : Attribuez les autorisations minimales nécessaires
2. **Audit régulier** : Passez en revue périodiquement les rôles et les accès des utilisateurs  
3. **Nomenclature claire** : Utilisez des noms de groupes descriptifs dans votre IdP
4. **Documentation** : Tenez à jour la documentation des attributions de rôle

#### Considérations de sécurité

1. **Attributs de rôle** : Assurez-vous que les attributs de rôle sont correctement sécurisés dans les réponses SAML
2. **Validation des attributs** : Vérifiez que seuls les systèmes autorisés peuvent attribuer des rôles
3. **Revues d'accès** : Passez régulièrement en revue les attributions des rôles administratifs
4. **Surveillance** : Surveillez les modifications de rôle et les actions administratives

### Résolution des problèmes de rôles

#### Problèmes courants

**Rôles non appliqués**:
- Vérifiez que les noms d'attribut SAML correspondent aux formats pris en charge
- Vérifiez que l'IdP envoie les informations de rôle
- Confirmez que les valeurs de rôle correspondent exactement aux noms de rôle FastComments

**Accès refusé**:
- Vérifiez que l'utilisateur a le rôle approprié attribué dans l'IdP
- Vérifiez l'orthographe des rôles et la sensibilité à la casse
- Confirmez que le rôle est correctement formaté dans la réponse SAML

**Autorisations manquantes**:
- Passez en revue les définitions de rôle et les autorisations requises
- Vérifiez s'il y a des affectations de rôles conflictuelles
- Vérifiez que l'utilisateur s'est connecté après les modifications de rôle