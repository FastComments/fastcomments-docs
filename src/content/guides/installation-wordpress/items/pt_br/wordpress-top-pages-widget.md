O widget Top Pages exibe as páginas com mais comentários do seu site. É útil para destacar seu conteúdo mais envolvente para novos visitantes e aumentar o tempo no site.

## Options

- **Title** (opcional): O cabeçalho mostrado acima da lista. Padrão: "Top Pages".

O widget Top Pages escolhe seu próprio layout com base nos dados disponíveis e não aceita um atributo count.

## How to Add It

### Inside a Post or Page

No editor de blocos, adicione um bloco **Shortcode** e cole:

[inline-code-attrs-start title = 'Shortcode Top Pages'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### In a Sidebar or Footer (Classic Themes)

Vá para **Appearance > Widgets** no admin do WordPress. No inseridor de blocos, pesquise por "FastComments" e escolha **FastComments: Top Pages**. Arraste-o para uma barra lateral, cabeçalho ou área de rodapé, então defina o título no painel do widget.

### In a Block Theme (Full Site Editing)

Abra o **Site Editor** em **Appearance > Editor**. Navegue até a parte do template onde o widget deve aparecer, insira um bloco **Legacy Widget** e selecione **FastComments: Top Pages** no menu suspenso.

## Troubleshooting

O widget só é renderizado depois que a configuração do FastComments estiver completa e um tenant ID estiver armazenado. Se a área do widget estiver em branco, conclua a configuração em **FastComments** no admin do WordPress e recarregue a página.