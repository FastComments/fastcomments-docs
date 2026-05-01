O widget Recent Discussions exibe as páginas do seu site com a atividade de comentários mais recente. É útil para destacar tópicos que ainda estão recebendo contribuições, para que os visitantes possam voltar a conversas ativas em vez de cair em páginas silenciosas.

## Opções

- **Título** (opcional): O cabeçalho exibido acima da lista. Padrão: "Discussões Recentes".
- **Contagem** (opcional): Quantas discussões mostrar. Intervalo de 1 a 50. Padrão: 20.

## Como Adicionar

### Dentro de um Post ou Página

No editor de blocos, adicione um bloco **Shortcode** e cole:

[inline-code-attrs-start title = 'Shortcode de Discussões Recentes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

O atributo `count` aceita qualquer valor entre 1 e 50.

### Em uma Barra Lateral ou Rodapé (Temas Clássicos)

Vá em **Aparência > Widgets** no admin do WordPress. No inseridor de blocos, pesquise por "FastComments" e escolha **FastComments: Recent Discussions**. Arraste-o para uma área de barra lateral, cabeçalho ou rodapé, então configure o título e a contagem no painel do widget.

### Em um Tema de Blocos (Edição Completa do Site)

Abra o **Editor do Site** em **Aparência > Editor**. Navegue até a parte do template onde o widget deve aparecer, insira um bloco **Legacy Widget** e selecione **FastComments: Recent Discussions** no menu dropdown.

## Solução de Problemas

O widget só é renderizado depois que a configuração do FastComments estiver completa e um tenant ID estiver armazenado. Se a área do widget estiver em branco, conclua a configuração em **FastComments** no admin do WordPress e recarregue a página.

Se a ordenação das discussões parecer desatualizada, verifique se as páginas subjacentes terminaram de sincronizar no painel do FastComments. O widget lê dados ao vivo, então comentários recém-importados podem demorar um pouco para aparecer.