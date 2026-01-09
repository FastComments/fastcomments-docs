[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Por padrão, novos comentários ao vivo aparecem no topo da lista de comentários à medida que são publicados em tempo real.

Quando essa opção está habilitada, novos comentários ao vivo serão adicionados ao final da lista. Isso afeta como os comentários aparecem quando são publicados ao vivo enquanto os usuários estão visualizando o tópico de comentários.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Com essa configuração habilitada:
- Novos comentários ao vivo postados por outros usuários aparecerão no final da lista de comentários
- Os usuários verão novos comentários aparecerem abaixo dos comentários existentes em tempo real
- Isso afeta apenas as atualizações de comentários ao vivo - não o carregamento inicial da página
- Isso pode ajudar a manter o fluxo de leitura quando os usuários estão acompanhando uma discussão

Observe que esta configuração afeta apenas onde novos comentários ao vivo são colocados conforme chegam em tempo real. Ela não afeta a ordem de classificação inicial quando a página é carregada.