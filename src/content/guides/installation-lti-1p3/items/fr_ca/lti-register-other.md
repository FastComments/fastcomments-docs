#### Sakai

Sakai prend en charge l'enregistrement dynamique LTI 1.3 sur les versions avec LTI Advantage. Depuis l'espace d'administration :

1. Connectez-vous en tant qu'administrateur Sakai et ouvrez l'**Administration Workspace**.
2. Choisissez **External Tools** > **Install LTI 1.3 Tool**.
3. Collez l'URL d'enregistrement FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-le ici</a>) et soumettez.
4. Approuvez l'outil lorsque l'échange est terminé.

L'outil apparaît alors sous **External Tools** et peut être ajouté aux sites par leurs responsables.

#### Schoology

Les instances Schoology Enterprise prennent en charge LTI 1.3, mais la disponibilité de l'enregistrement dynamique varie selon le déploiement. Vérifiez auprès de votre responsable de compte Schoology.

Si l'enregistrement dynamique n'est pas disponible sur votre instance Schoology, vous devrez configurer l'intégration manuellement en utilisant ces points de terminaison :

- **OIDC Login URL**: `https://fastcomments.com/lti/v1p3/login`
- **Target Link URL**: `https://fastcomments.com/lti/v1p3/launch`
- **Public Keyset URL (JWKS)**: `https://fastcomments.com/lti/v1p3/jwks`
- **Redirect URLs**: `https://fastcomments.com/lti/v1p3/launch`

Après que Schoology vous a fourni un Client ID et un Deployment ID, contactez le support FastComments pour enregistrer la configuration sur votre tenant.

#### Autres plateformes LTI 1.3

Tout LMS qui respecte la spécification IMS LTI 1.3 Advantage devrait fonctionner avec la même URL d'enregistrement (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-le ici</a>). Recherchez un paramètre libellé "Dynamic Registration", "Tool Registration URL", "Tool initiation registration endpoint", ou similaire.

Si votre plateforme ne prend en charge que la configuration manuelle de LTI 1.3, utilisez les quatre points de terminaison énumérés dans la section Schoology ci‑dessus et contactez le support pour finaliser.