Un objet `Page` représente la page à laquelle plusieurs commentaires peuvent appartenir. Cette relation est définie par
`urlId`.

Une `Page` stocke des informations telles que le titre de la page, le nombre de commentaires et `urlId`.

La structure de l'objet Page est la suivante :

[inline-code-attrs-start title = 'Structure de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Page {
    id: string
    urlId: string
    url: string
    title?: string
    createdAt: string
    commentCount: number
    rootCommentCount: number
    /** Setting this to null means all SSO users can see the page. An empty list means it is closed to all users. **/
    accessibleByGroupIds?: string[] | null
    /** Is this page closed for new comments? **/
    isClosed?: boolean
}
[inline-code-end]
