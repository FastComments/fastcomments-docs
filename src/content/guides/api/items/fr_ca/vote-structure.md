Un objet `Vote` représente un vote laissé par un utilisateur.

La relation entre les commentaires et le vote est définie via `commentId`.

La structure de l'objet Vote est la suivante :

[inline-code-attrs-start title = 'Structure de Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Vote {
    id: string
    urlId: string
    commentId: string
    userId: string
    direction: 1 | -1
    createdAt: string
}
[inline-code-end]
