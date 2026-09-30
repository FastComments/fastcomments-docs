A borda ao redor da caixa de comentário é composta pela própria borda da área de texto mais algumas linhas finas que o widget desenha ao seu redor. Para mudar sua cor ou o arredondamento dos cantos, defina essas variáveis CSS em vez de estilizar o `textarea` diretamente. As variáveis restilizam todas as partes da borda de uma vez, de modo que os cantos e as cores sempre se alinhem.

| Variável | O que altera | Padrão |
|---|---|---|
| `--fc-input-border-color` | Cor da borda | `#bfbfbf` |
| `--fc-input-border-color-focus` | Cor da borda enquanto o usuário está digitando | `#555` |
| `--fc-input-border-radius` | Arredondamento dos cantos arredondados | `11px` |
| `--fc-input-border-start-start-radius` | Arredondamento do canto superior esquerdo quadrado (superior direito em idiomas da direita para a esquerda) | `0` |

Adicione o CSS na caixa **Custom CSS** na [página de Personalização do Widget](https://fastcomments.com/auth/my-account/customize-widget), ou passe‑o com a opção `customCSS`. Você só precisa definir as variáveis que deseja alterar.

Você também pode usar o assistente **Comment box border** logo abaixo da caixa Custom CSS, que gera esse CSS para você.

## Alterar a Cor da Borda

[inline-code-attrs-start title = 'Cor da Borda'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Alterar a Cor da Borda ao Digitar

[inline-code-attrs-start title = 'Cor da Borda ao Digitar'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Arredondar Todos os Quatro Cantos

Por padrão o canto superior esquerdo é quadrado. Defina ambas as variáveis de raio para arredondar os quatro cantos da mesma forma:

[inline-code-attrs-start title = 'Arredondar Todos os Quatro Cantos'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Cantos Quadrados

[inline-code-attrs-start title = 'Cantos Quadrados'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Combinar com sua Marca

[inline-code-attrs-start title = 'Cores da Marca e Cantos'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Cores Diferentes no Modo Escuro

Quando o widget está no modo escuro ele tem a classe `dark`, então você pode definir valores diferentes para o modo escuro:

[inline-code-attrs-start title = 'Cores da Borda no Modo Escuro'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Por que não estilizar a área de texto diretamente?

O widget desenha parte da borda da caixa de comentário ele mesmo, ao redor da área de texto. Se você definir `border-color` ou `border-radius` apenas no `textarea`, essas linhas mantêm o estilo padrão e a borda parece incompatível, por exemplo, uma linha quadrada atravessando um canto arredondado. As variáveis acima alteram ambos de uma vez.