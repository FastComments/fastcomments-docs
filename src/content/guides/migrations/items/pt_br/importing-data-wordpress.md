Nosso [plugin WordPress](https://wordpress.org/plugins/fastcomments/) possui um poderoso mecanismo de importação baseado em UI. Ao instalar o plugin,
ele irá guiá-lo através da vinculação da sua instalação WordPress com o FastComments e da cópia dos seus dados de comentários existentes.

**Isso é feito sem copiar ou baixar nada manualmente.**

O processo de migração será indicado a você através da UI durante a migração. A maioria das migrações leva apenas alguns minutos.

O mecanismo foi projetado para não sobrecarregar excessivamente sua instalação WordPress durante a migração.

Se você está movendo seu site para fora do WordPress, você pode importar uma exportação XML ou CSV do WordPress em vez de usar o plugin. Veja
[Movendo Seus Comentários para um Novo Site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & Firewalls

Para que a configuração automatizada do WordPress funcione, precisamos fazer chamadas para a sua instalação WordPress.
Firewalls como o Cloudflare podem nos bloquear e causar falha na integração. Nesses casos, [podemos fornecer a você](https://fastcomments.com/auth/my-account/help) um conjunto de IPs para colocar na lista de permissões para a integração.

### Propriedade dos Dados

No caso da nossa migração WordPress, quaisquer dados de comentários novos ou atualizados são sincronizados automaticamente de volta para a sua instalação WordPress
nos bastidores. Isso significa que, enquanto os comentários são servidos pelo próprio FastComments para tirar a carga da sua implantação WordPress,
nós **também** os salvamos no seu banco de dados como backup. Isso também significa que, se você desejar mudar do FastComments, seus dados já
estarão migrados e atualizados.