Un objet `EmailTemplate` représente la configuration d'un modèle d'email personnalisé, pour un locataire.

Le système sélectionnera le modèle d'email à utiliser via :

- Son identifiant de type, nous l'appelons `emailTemplateId`. Ce sont des constantes.
- Le `domain`. Nous essaierons d'abord de trouver un modèle pour le domaine auquel l'objet associé (comme un `Comment`) est lié, et si aucune correspondance n'est trouvée, nous essaierons de trouver un modèle où le domaine est null ou `*`.

La structure de l'objet `EmailTemplate` est la suivante :

[inline-code-attrs-start title = 'Structure de Modèle d’Email'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** READONLY **/
    createdAt: string
    /** READONLY **/
    updatedAt: string
    /** READONLY **/
    updatedByUserId: string
    /** The domain the template should be associated with. **/
    domain?: string | '*' | null
    /** The email template content in EJS syntax. **/
    ejs: string
    /** A map of overridden translation keys to values, for each supported locale. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** An object that represents the render context of the template. **/
    testData: object
}
[inline-code-end]

### Notes

- Vous pouvez obtenir les valeurs valides de `emailTemplateId` depuis l'endpoint `/definitions`.
- L'endpoint `/definitions` inclut également les traductions et données de test par défaut.
- Les modèles échoueront à l'enregistrement avec une structure ou des données de test invalides.
