O widget Top Pages exibe as páginas mais comentadas do seu site. É útil para destacar seu conteúdo mais envolvente para novos visitantes e aumentar o tempo de permanência no site.

## Opções

- **Title** (opcional): O título exibido acima da lista. O padrão é "Top Pages".

O widget Top Pages escolhe seu próprio layout com base nos dados disponíveis e não aceita um atributo count.

## Como Adicioná-lo

### Dentro de um Post ou Página

No editor de blocos, adicione um bloco **Shortcode** e cole:

[inline-code-attrs-start title = 'Shortcode do Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### Em uma Barra Lateral ou Rodapé (Temas Clássicos)

Vá para **Appearance > Widgets** no seu painel administrativo do WordPress. No inseridor de blocos, pesquise por "FastComments" e escolha **FastComments: Top Pages**. Arraste-o para uma área de barra lateral, cabeçalho ou rodapé e, em seguida, defina o título no painel do widget.

### Em um Tema de Blocos (Edição Completa do Site)

Abra o **Site Editor** em **Appearance > Editor**. Navegue até a parte do template onde o widget deve aparecer, insira um bloco **Legacy Widget** e selecione **FastComments: Top Pages** no menu suspenso.

## Solução de Problemas

O widget só é renderizado após a conclusão da configuração do FastComments e o armazenamento de um tenant ID. Se a área do widget estiver em branco, conclua a configuração em **FastComments** no painel administrativo do WordPress e recarregue a página.