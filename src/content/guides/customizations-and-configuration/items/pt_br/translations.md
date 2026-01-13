[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Com o FastComments, todo o texto no widget de comentários é personalizável.

Você pode substituir uma única parte do texto, como o botão de envio, ou todo o texto em todo o widget de comentários.

Por padrão, o texto no widget de comentários é traduzido com base na localidade do usuário. No entanto, podemos substituir o texto se estivermos confiantes de que nossa base de usuários está usando a mesma localidade/idioma, por exemplo:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Todas as traduções personalizáveis podem ser encontradas <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">aqui</a> na aba "opções avançadas".

No entanto, há uma maneira mais fácil, via a interface de personalização do widget. Lá, podemos simplesmente encontrar o texto que aparece no widget de comentários na localidade EN_US e especificar uma substituição.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Todas as substituições de tradução afetam atualmente todas as localidades.

---