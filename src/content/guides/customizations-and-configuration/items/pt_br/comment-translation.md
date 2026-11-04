Por padrão, o FastComments traduz a própria interface do widget (botões, rótulos, mensagens de erro) para o idioma do visualizador, mas os comentários são exibidos exatamente como foram escritos.

A Tradução de Comentários adiciona a tradução dos próprios comentários, para que uma comunidade que escreve em mais de um idioma possa ler cada comentário no seu próprio.

Há dois modos:

- **Automático**: comentários escritos em um idioma diferente do idioma do widget são exibidos traduzidos. Uma linha "Traduzido do espanhol" aparece abaixo do comentário com um link **Mostrar original** que alterna entre a tradução e o texto não traduzido.
- **Manual**: cada comentário recebe um link **Traduzir**. Ao clicar nele, aquele comentário é traduzido para o idioma do visualizador, com o mesmo alternador **Mostrar original** depois. Comentários que já estão no idioma do visualizador indicam isso em vez disso.

O idioma de destino é o idioma do widget: a configuração [Locale / Language](#locale) na mesma regra de personalização, se estiver definida, caso contrário, o idioma do navegador do visitante. Nada é exigido do visitante, sem conta, extensão ou software adicional, e funciona da mesma forma no widget incorporado e nas páginas de comentários independentes.

Isso é ativado por <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">regra de personalização</a>, então pode se aplicar ao seu site inteiro, a um domínio ou a uma única página. Fica desativado por padrão.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Menu suspenso de Tradução de Comentários na página de personalização do widget com as opções Desativado, Automático e Manual'; title='Tradução de Comentários' app-screenshot-end]

A tradução é uma configuração do lado do servidor. Passar `commentTranslation` na configuração JavaScript do widget não a ativa; a regra sim.

Cada comentário é traduzido no máximo uma vez por idioma de destino e o resultado é armazenado com o comentário, então um comentário lido por mil falantes de espanhol é traduzido uma vez. Se o autor ou um moderador editar o comentário, a tradução armazenada é descartada e o comentário é traduzido novamente na próxima visualização. O texto traduzido passa pelas mesmas regras de sanitização e de links que o comentário original. Comentários muito longos, ou cujo idioma não pode ser detectado, são exibidos sem tradução.

O uso é cobrado como tokens de LLM pela mesma taxa dos [AI Agents](/guide-ai-agents.html#plans-and-eligibility) e aparece no mesmo lugar nos seus detalhes de faturamento.