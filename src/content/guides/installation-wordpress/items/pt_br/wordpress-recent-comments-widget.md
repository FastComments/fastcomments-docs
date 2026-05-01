---
O widget Recent Comments exibe os comentários mais recentes publicados em todo o seu site. É útil em barras laterais, rodapés ou em qualquer lugar onde você queira destacar atividade recente para incentivar mais leitura.

## Opções

- **Title** (optional): O cabeçalho exibido acima da lista. Padrão: "Recent Comments".
- **Count** (optional): Quantos comentários exibir. Intervalo de 1 a 50. Padrão: 5.

## Como Adicioná-lo

### Dentro de um Post ou Página

No editor de blocos, adicione um bloco **Shortcode** e cole:

[inline-code-attrs-start title = 'Shortcode de Comentários Recentes'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

O atributo `count` aceita qualquer valor entre 1 e 50.

### Em uma Barra Lateral ou Rodapé (Temas Clássicos)

Vá para **Aparência > Widgets** no painel do WordPress. No inseridor de blocos, pesquise por "FastComments" e escolha **FastComments: Recent Comments**. Arraste-o para uma área de barra lateral, cabeçalho ou rodapé, e então configure o título e a contagem no painel do widget.

### Em um Tema de Blocos (Edição completa do site)

Abra o **Editor do Site** em **Aparência > Editor**. Navegue até a parte do template onde o widget deve aparecer, insira um bloco **Legacy Widget**, e selecione **FastComments: Recent Comments** no menu suspenso.

## Solução de problemas

O widget só é exibido depois que a configuração do FastComments estiver concluída e um tenant ID for armazenado. Se a área do widget estiver em branco, conclua a configuração em **FastComments** no painel do WordPress e recarregue a página.

---