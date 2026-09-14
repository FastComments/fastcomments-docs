`tenantId: "demo"` est un bac à sable public partagé. Il fonctionne sans inscription, c’est pourquoi les exemples l’utilisent, mais tous les autres utilisateurs de FastComments écrivent dans les mêmes fils et n’importe qui peut les modérer. Changez-le avant de publier quoi que ce soit d’important.

Votre ID de locataire se trouve sur la [page des secrets API](https://fastcomments.com/auth/my-account/api-secret).

Un ID de locataire est public et doit être présent dans le code du navigateur. Un secret API ne l’est pas, et rien sur cette page n’en a besoin.

## Lire depuis une variable d’environnement

Les vals de Val Town sont publiques dans le niveau gratuit, donc leur source est lisible par tous. Conservez tout ce qui est sensible dans des variables d’environnement, lues avec `Deno.env.get` :

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Seuls les comptes créés sur eu.fastcomments.com définissent cela, à "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Cela importe plus que d’habitude sur Val Town pour une deuxième raison : **remixer un val copie les clés des variables d’environnement, mais pas leurs valeurs.** Un secret stocké dans une variable d’environnement ne suit pas votre val dans le compte d’un autre utilisateur. Un secret écrit dans un fichier le fait.

Revenir à `"demo"` permet au val de fonctionner pour quiconque le remixe avant de définir son propre locataire.

## Comptes EU

Un compte, ses données et ses clés résident dans une seule région. Si le vôtre a été créé sur `eu.fastcomments.com`, chaque configuration de widget doit également contenir `region: "eu"`, et les scripts se chargent depuis `cdn-eu.fastcomments.com`. Sinon, laissez les deux tels quels.

---