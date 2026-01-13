In FastComments chamamos o id do artigo, ou da página ao qual os comentários ficam vinculados, de URL ID, pois pode ser uma url ou um ID.
Defina o URL ID da seguinte maneira. O componente monitora mudanças no objeto config e irá recarregar, então você pode apenas atualizar as configurações "url" e "urlId".

Veja um exemplo completo funcionando [aqui](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

Execute o exemplo de paginação via:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Região da Conta (ATENÇÃO: Clientes da UE)

Se sua conta estiver localizada na UE, defina `region = 'eu'` na configuração do widget, por exemplo:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

Caso contrário, você não precisa definir `region`.