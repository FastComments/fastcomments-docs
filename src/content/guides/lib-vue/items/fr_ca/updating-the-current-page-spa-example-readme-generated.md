Dans FastComments, nous appelons l'ID d'article, ou la page à laquelle les commentaires sont rattachés, l'ID d'URL car il peut s'agir d'une URL ou d'un ID.
Définissez l'ID d'URL de la manière suivante. Le composant surveille les modifications de l'objet de configuration et se rechargera, vous pouvez donc simplement mettre à jour les paramètres "url" et "urlId".

Voir un exemple complet fonctionnel [ici](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Exécutez l'exemple de pagination via :

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

Sinon, vous n'avez pas à définir `region`.