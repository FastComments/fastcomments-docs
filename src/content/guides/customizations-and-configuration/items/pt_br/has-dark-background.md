[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Por padrão, o widget de comentários FastComments detectará automaticamente o modo escuro na maioria dos sites.

Quando o modo escuro é detectado, o FastComments mudará do texto preto em fundos brancos para texto branco em um fundo preto. Imagens também serão alteradas.

Ao carregar a página, o widget tentará determinar quão escuro é o plano de fundo da página por trás do widget de comentários. Isso significa que
a página pode ter um fundo branco, mas se você colocar o widget de comentários dentro de um contêiner com um fundo preto, o modo escuro ainda deverá
ser ativado automaticamente para tornar os comentários legíveis.

No entanto, o mecanismo de detecção, que depende de determinar a "luminância", pode não ativar o modo escuro quando você desejar. Para forçá-lo a ativar, defina a
flag *hasDarkBackground* como true da seguinte forma:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---