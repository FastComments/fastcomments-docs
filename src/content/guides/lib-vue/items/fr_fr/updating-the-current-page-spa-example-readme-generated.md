Dans FastComments nous appelons l'identifiant de l'article, ou la page à laquelle les commentaires sont liés, l'URL ID car il peut s'agir d'une URL ou d'un identifiant.
Définissez l'URL ID de la manière suivante. Le composant surveille les modifications de l'objet config et se rechargera, vous pouvez donc simplement mettre à jour les paramètres "url" et "urlId".

Voir un exemple complet et fonctionnel [ici](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Exécutez l'exemple de pagination avec :

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Région du compte (ATTENTION : clients de l'UE)

Si votre compte est situé dans l'UE, définissez `region = 'eu'` dans la configuration du widget, par exemple :

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Dans le cas contraire, vous n'avez pas besoin de définir `region`.