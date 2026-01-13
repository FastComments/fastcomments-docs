Un objet `HashTag` représente un tag qui peut être laissé par un utilisateur. Les HashTags peuvent être utilisés pour créer un lien vers un contenu externe ou pour
relier des commentaires connexes ensemble.

La structure de l'objet `HashTag` est la suivante :

[inline-code-attrs-start title = 'Structure de HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Notes :

- Dans certains endpoints API, vous verrez que le hashtag est utilisé dans l'URL. N'oubliez pas d'encoder les valeurs en URI. Par exemple, `#` devrait être représenté comme `%23`.
- Certains de ces champs sont marqués `READONLY` - ils sont retournés par l'API mais ne peuvent pas être définis.

