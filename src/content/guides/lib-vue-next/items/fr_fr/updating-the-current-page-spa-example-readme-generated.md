Dans FastComments, nous appelons l'identifiant de l'article, ou la page à laquelle les commentaires sont attachés, l'URL ID car il peut s'agir d'une URL ou d'un identifiant.
Définissez l'URL ID de la manière suivante. Le composant surveille les modifications de l'objet config, et se rechargera, vous pouvez donc mettre à jour l'URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Région du compte (ATTENTION : clients de l'UE)

Si votre compte est situé dans l'UE, définissez `region = 'eu'` dans la configuration du widget, par exemple :

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Sinon, vous n'avez pas à définir `region`.