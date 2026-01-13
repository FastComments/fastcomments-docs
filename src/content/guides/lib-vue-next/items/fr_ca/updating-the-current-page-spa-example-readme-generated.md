Dans FastComments nous appelons l'id de l'article, ou la page à laquelle les commentaires sont rattachés, le URL ID car cela peut être une url ou un ID.
Définissez le URL ID de la manière suivante. Le composant surveille les modifications de l'objet config, et se rechargera, donc vous pouvez mettre à jour le URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Région du compte (ATTENTION : clients de l'UE)

Si votre compte est situé dans l'UE, définissez `region = 'eu'` dans la configuration du widget, par exemple :

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Sinon, vous n'avez pas à définir `region`.