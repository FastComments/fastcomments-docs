Nosso [plugin do WordPress](https://wordpress.org/plugins/fastcomments/) possui um poderoso mecanismo de importação baseado em interface. Ao instalar o plugin,
ele guiará você no processo de vincular sua instalação do WordPress ao FastComments e copiar seus comentários existentes.

**Isso é feito sem copiar ou baixar nada manualmente.**

O processo de migração será indicado a você pela interface durante a migração. A maioria das migrações leva apenas alguns minutos.

O mecanismo foi projetado para não sobrecarregar sua instalação do WordPress durante a migração.

### CloudFlare & Firewalls

Para que a configuração automatizada do WordPress funcione, precisamos fazer chamadas à sua instalação do WordPress.
Firewalls como o Cloudflare podem nos bloquear e fazer com que a integração falhe. Nesses casos, [podemos fornecer
a você](https://fastcomments.com/auth/my-account/help) um conjunto de IPs para colocar na lista de permissões para a integração.

### Propriedade dos Dados

No caso da nossa migração do WordPress, quaisquer dados de comentários novos ou atualizados são automaticamente sincronizados de volta para sua instalação do WordPress
nos bastidores. Isso significa que, enquanto os comentários são servidos pelo próprio FastComments para reduzir a carga da sua implantação do WordPress,
nós **também** os salvamos em seu banco de dados como backup. Isso também significa que, se você desejar mudar do FastComments, seus dados já estão
migrados e atualizados.