Nosso [Plugin WordPress](https://wordpress.org/plugins/fastcomments/) tem um poderoso mecanismo de importação baseado em UI. Ao instalar o plugin,
ele o guiará através da vinculação da sua instalação WordPress com o FastComments e da cópia dos seus dados de comentários existentes.

**Isso é feito sem copiar ou baixar nada manualmente.**

O processo de migração será indicado a você via UI durante a migração. A maioria das migrações leva apenas alguns minutos.

O mecanismo foi projetado para não sobrecarregar excessivamente sua instalação WordPress durante a migração.

Se você está movendo seu site fora do WordPress, pode importar um export XML ou CSV do WordPress em vez de usar o plugin. Veja
[Movendo seus comentários para um novo site](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare e FireWalls

Para que a configuração automática do WordPress funcione, precisamos fazer chamadas à sua instalação WordPress.
Firewalls como o Cloudflare podem nos bloquear e causar falha na integração. Nesses casos, [podemos fornecer
a você](https://fastcomments.com/auth/my-account/help) um conjunto de IPs para colocar na lista branca para a integração.

### Propriedade dos Dados

No caso da nossa migração WordPress, quaisquer novos ou atualizados dados de comentários são sincronizados automaticamente de volta à sua instalação WordPress
nos bastidores. Isso significa que, enquanto os comentários são servidos pelo próprio FastComments para reduzir a carga da sua implantação WordPress,
nós **também** os salvamos em seu banco de dados como backup. Isso também significa que, se você desejar mudar do FastComments, seus dados já estão
migrados e atualizados.