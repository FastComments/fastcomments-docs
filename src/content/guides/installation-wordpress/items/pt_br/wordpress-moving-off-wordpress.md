Se você está movendo seu site do WordPress e quer FastComments no novo site, não precisa do plugin WordPress. Exporte seus comentários do WordPress, então faça upload do arquivo na [Página de Importação](https://fastcomments.com/auth/my-account/manage-data/import) no painel do FastComments.

Nós suportamos dois formatos de exportação do WordPress.

### WordPress XML (Recomendado)

Este é o arquivo do exportador interno do WordPress, portanto nenhum plugin extra é necessário.

1. No seu admin do WordPress, vá em `Tools -> Export`.
2. Selecione `All content` e clique em `Download Export File`.
3. Na FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), selecione `WordPress (.xml)` e faça upload do arquivo.

Cada comentário está vinculado à URL da postagem onde foi deixado, que já está no arquivo.

A importação mantém o nome do autor, e‑mail e site, a data, o conteúdo, a estrutura de respostas e se o comentário foi aprovado. Avatares dos comentaristas são trazidos do Gravatar. Votos não fazem parte deste formato.

### WordPress CSV

Este é o arquivo do [WebToffee's WordPress Comments Import & Export plugin](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Instale o plugin no admin do WordPress e exporte seus comentários como CSV.
2. Substitua cada valor `comment_post_ID` pela URL da postagem.
3. Na FastComments [Import page](https://fastcomments.com/auth/my-account/manage-data/import), selecione `WordPress (.csv)` e faça upload do arquivo.

Cada comentário está vinculado à coluna `comment_post_ID`. O WordPress preenche essa coluna com o ID da postagem, e seu novo site não tem IDs de postagens do WordPress, então o passo 2 a substitui pela URL.

A importação mantém o nome do autor, e‑mail e site, a data, o conteúdo, a estrutura de respostas e se o comentário foi aprovado. Avatares dos comentaristas são trazidos do Gravatar. Também mantém a flag de spam do WordPress e os likes e dislikes do wpDiscuz quando o arquivo os inclui.

### Correspondendo Comentários às Suas Novas Páginas

Se seu novo site mantém as mesmas URLs do seu site WordPress, os comentários aparecerão nas páginas correspondentes sem configuração extra.

Se o domínio mudar, execute a [Domain Migration tool](/guide-migrations.html#migrating-domains) após a importação. Se URLs individuais de páginas mudarem, você pode [migrate each page](/guide-migrations.html#migrating-pages) de sua URL antiga para a nova.

Para migrações em massa de páginas, como remover o domínio do valor que você passa ao campo [urlId](/guide-customizations-and-configuration.html#url-id) do widget de comentários, [open a support ticket](https://fastcomments.com/auth/my-account/help) e nós cuidaremos disso para você.

### Antes de Trocar

Você pode executar a importação quantas vezes quiser. Reimportar o mesmo arquivo [does not create duplicates](/guide-migrations.html#importing-data), então você pode importar uma vez para testar o novo site, depois importar novamente com seus comentários mais recentes logo antes de mudar.

Para arquivos de exportação maiores que 1 GB, [reach out to support](https://fastcomments.com/auth/my-account/help).

Para adicionar FastComments ao seu novo site, veja o [Installation guide](/guide-installation.html).