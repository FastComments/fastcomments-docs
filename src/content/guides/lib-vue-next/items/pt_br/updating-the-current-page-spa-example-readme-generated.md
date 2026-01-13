---
No FastComments, chamamos o article id, ou a página à qual os comentários são vinculados, de URL ID, pois pode ser uma url ou um ID.
Defina o URL ID da seguinte maneira. O componente fica observando alterações no objeto config e irá recarregar, então você pode atualizar o URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### Região da Conta (ATENÇÃO: Clientes da UE)

Se sua conta estiver localizada na UE, defina `region = 'eu'` na configuração do widget, por exemplo:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

Caso contrário, você não precisa definir `region`.
---