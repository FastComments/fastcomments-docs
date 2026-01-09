Ce guide couvre les problèmes courants d'authentification SAML et leurs solutions.

### Certificate and Security Issues

#### Invalid Certificate Error

**Symptoms**:
- Erreur « Certificate validation failed »
- Les utilisateurs ne peuvent pas terminer l'authentification SAML
- Les réponses SAML sont rejetées

**Common Causes**:
- Le format du certificat est incorrect
- Le certificat a expiré
- Le mauvais certificat a été fourni
- Caractères ou espaces supplémentaires dans le certificat

**Solutions**:
1. **Verify Certificate Format**:
   - Vérifiez que le certificat contient les marqueurs `-----BEGIN CERTIFICATE-----` et `-----END CERTIFICATE-----`
   - Supprimez tout espace excessif ou saut de ligne
   - Copiez le certificat directement depuis les métadonnées ou la configuration de l'IdP

2. **Check Certificate Validity**:
   - Vérifiez que le certificat n'a pas expiré
   - Confirmez que le certificat correspond au bon IdP
   - Utilisez des validateurs de certificats en ligne pour vérifier le format

3. **Re-download Certificate**:
   - Téléchargez un certificat récent depuis l'IdP
   - Utilisez l'URL des métadonnées de l'IdP si disponible
   - Confirmez que le certificat correspond à la configuration actuelle de l'IdP

#### Signature Verification Failed

**Symptoms**:
- Erreurs de validation de la signature de l'assertion SAML
- L'authentification échoue après la connexion à l'IdP
- Messages d'erreur « Invalid signature »

**Solutions**:
1. **Algorithm Mismatch**:
   - Vérifiez que l'algorithme de signature dans FastComments correspond à l'IdP
   - Essayez différents algorithmes de signature (SHA-256, SHA-1, SHA-512)
   - Vérifiez que l'algorithme de digest correspond à la configuration de l'IdP

2. **Certificate Issues**:
   - Assurez-vous que le certificat de signature correct est configuré
   - Vérifiez que le certificat correspond à la clé privée utilisée par l'IdP
   - Vérifiez s'il y a eu une rotation de certificat dans l'IdP

### Configuration Issues

#### Wrong Entity ID or ACS URL

**Symptoms**:
- L'IdP signale « Unknown Service Provider »
- Les réponses SAML sont envoyées au mauvais endpoint
- L'authentification ne se termine pas

**Solutions**:
1. **Verify SP Information**:
   - Copiez l'Entity ID exact depuis la configuration FastComments
   - Assurez-vous que l'ACS URL correspond au format : `https://fastcomments.com/saml/callback/{tenant-id}`
   - Vérifiez les fautes de frappe dans l'ID de tenant

2. **IdP Configuration**:
   - Mettez à jour l'IdP avec l'Entity ID SP correct
   - Configurez l'ACS/Reply URL approprié
   - Vérifiez les paramètres de binding de l'IdP (HTTP-POST recommandé)

#### Missing or Incorrect Attributes

**Symptoms**:
- Utilisateurs créés sans rôles appropriés
- Informations de profil utilisateur manquantes
- Erreurs « Email required »

**Solutions**:
1. **Email Attribute**:
   - Assurez-vous que l'IdP envoie l'attribut email
   - Vérifiez le mappage du nom d'attribut (email, emailAddress, etc.)
   - Vérifiez que la valeur email est une adresse email valide

2. **Role Attributes**:
   - Confirmez que l'IdP envoie les informations de rôle/groupe
   - Vérifiez que les noms d'attributs de rôle correspondent aux attentes de FastComments
   - Vérifiez que les valeurs de rôle correspondent exactement aux noms de rôle FastComments

3. **Attribute Format**:
   - Testez à la fois les formats tableau et séparés par des virgules pour les rôles
   - Assurez-vous que les valeurs d'attribut n'ont pas d'espaces superflus
   - Vérifiez la sensibilité à la casse dans les noms de rôle

### Authentication Flow Issues

#### Redirect Loop

**Symptoms**:
- Le navigateur redirige indéfiniment entre FastComments et l'IdP
- L'authentification ne se termine jamais
- Plusieurs redirections affichées dans les outils de développement du navigateur

**Solutions**:
1. **Check SP Configuration**:
   - Vérifiez que l'Entity ID correspond exactement à la configuration de l'IdP
   - Assurez-vous que l'ACS URL est correctement configurée dans l'IdP
   - Vérifiez la présence de slashs finaux dans les URLs

2. **Session Issues**:
   - Effacez les cookies du navigateur et réessayez
   - Testez en fenêtre de navigation privée/incognito
   - Vérifiez les paramètres de durée de session

#### Access Denied After Authentication

**Symptoms**:
- L'authentification SAML réussit
- L'utilisateur est redirigé vers FastComments
- Message « Access denied » ou erreur de permissions affichée

**Solutions**:
1. **Role Assignment**:
   - Vérifiez que l'utilisateur a les rôles appropriés dans l'IdP
   - Vérifiez que l'attribut de rôle est envoyé dans la réponse SAML
   - Confirmez que les noms de rôle correspondent exactement aux exigences de FastComments

2. **Package Limitations**:
   - Vérifiez que le compte dispose d'un forfait Flex ou Pro
   - Vérifiez que la fonctionnalité SAML est activée pour le forfait
   - Contactez le support si le forfait inclut SAML mais que la fonctionnalité est indisponible

### Identity Provider Specific Issues

#### Microsoft Azure AD

**Common Issues**:
- Les attributions de rôles d'application ne se reflètent pas dans les tokens
- Les claims ne sont pas envoyés correctement
- Exigences d'affectation d'utilisateurs

**Solutions**:
- Vérifiez l'affectation des utilisateurs à l'application FastComments
- Vérifiez que les rôles d'application sont correctement configurés
- Assurez-vous que le mappage des claims inclut les attributs requis

#### Okta

**Common Issues**:
- Les filtres de groupe ne fonctionnent pas correctement
- Déclarations d'attributs mal configurées
- Problèmes d'affectation d'application

**Solutions**:
- Passez en revue la configuration des déclarations d'attributs
- Vérifiez l'affectation de groupe et les règles de filtrage
- Vérifiez que l'application est affectée aux utilisateurs/groupes appropriés

#### Google Workspace

**Common Issues**:
- Les attributs personnalisés ne sont pas mappés correctement
- L'appartenance aux groupes n'est pas envoyée
- Erreurs de configuration de l'application SAML

**Solutions**:
- Configurez un schéma personnalisé pour les attributs de rôle
- Vérifiez la propagation de l'appartenance aux groupes
- Vérifiez le mappage des attributs de l'application SAML

### Network and Connectivity Issues

#### Timeout Errors

**Symptoms**:
- Le processus d'authentification dépasse le délai
- Erreurs « Request timeout » ou similaires
- Flux d'authentification lent

**Solutions**:
1. **Network Connectivity**:
   - Vérifiez que les règles de pare-feu autorisent la communication avec FastComments
   - Vérifiez la résolution DNS pour fastcomments.com
   - Testez la connectivité réseau depuis l'IdP vers FastComments

2. **Performance Issues**:
   - Vérifiez les temps de réponse de l'IdP
   - Vérifiez que la validation de la chaîne de certificats n'est pas lente
   - Tenez compte de la latence réseau entre l'IdP et les utilisateurs

#### SSL/TLS Issues

**Symptoms**:
- Avertissements de certificat pendant l'authentification
- Échecs de négociation SSL
- Erreurs « Secure connection failed »

**Solutions**:
- Assurez-vous que tous les endpoints SAML utilisent HTTPS
- Vérifiez la validité des certificats pour tous les domaines impliqués
- Vérifiez la compatibilité des versions TLS

### Debugging and Logging

#### Enabling Debug Information

1. **Browser Developer Tools**:
   - Surveillez l'onglet Network pendant le flux SAML
   - Vérifiez la Console pour les erreurs JavaScript
   - Examinez les requêtes POST SAML (si visibles)

2. **IdP Logging**:
   - Activez le débogage SAML dans votre IdP
   - Consultez les logs de l'IdP pour les détails des requêtes/réponses SAML
   - Recherchez les problèmes de mappage d'attributs

#### Common Log Messages

**FastComments Logs**:
- « SAML config not found » - SAML non activé ou mal configuré
- « Invalid certificate » - Échec de la validation du certificat
- « Missing email attribute » - Email requis non fourni dans la réponse SAML

**IdP Logs**:
- « Unknown service provider » - Incompatibilité de l'Entity ID
- « Invalid ACS URL » - URL du Assertion Consumer Service incorrecte
- « User not assigned » - L'utilisateur n'a pas accès à l'application SAML

### Getting Help

#### Information to Gather

Lorsque vous contactez le support, fournissez :
- Messages d'erreur exacts et horodatages
- Détails de la configuration SAML (sans données sensibles)
- Type et version de l'IdP
- Étapes pour reproduire le problème
- Informations sur le navigateur et le réseau

#### FastComments Support

Pour les problèmes liés à SAML :
1. Utilisez le [portail d'assistance](https://fastcomments.com/auth/my-account/help)
2. Incluez l'ID du tenant et les emails des utilisateurs affectés
3. Fournissez les messages d'erreur et les détails de configuration
4. Spécifiez le type d'IdP et l'approche de configuration

#### IdP Support

Pour les problèmes spécifiques à l'IdP :
- Consultez la documentation de l'IdP pour le dépannage SAML
- Utilisez les canaux de support de l'IdP pour les problèmes de configuration
- Exploitez les forums communautaires de l'IdP pour les problèmes courants

### Prevention Tips

#### Best Practices

1. **Test Thoroughly**:
   - Testez les modifications de configuration dans un environnement non production
   - Vérifiez avec plusieurs utilisateurs de test
   - Documentez les configurations fonctionnelles

2. **Monitor Regularly**:
   - Mettez en place une surveillance des échecs d'authentification SAML
   - Vérifiez les dates d'expiration des certificats
   - Surveillez les modifications de configuration de l'IdP

3. **Documentation**:
   - Tenez à jour la documentation de la configuration SAML
   - Documentez toute configuration ou solution de contournement personnalisée
   - Conservez les coordonnées des administrateurs de l'IdP

#### Proactive Maintenance

1. **Certificate Management**:
   - Surveillez les dates d'expiration des certificats
   - Planifiez les procédures de rotation des certificats
   - Testez les mises à jour de certificats avant expiration

2. **Configuration Reviews**:
   - Passez régulièrement en revue la configuration SAML
   - Vérifiez que la configuration de l'IdP reste à jour
   - Mettez à jour la documentation au fur et à mesure des changements