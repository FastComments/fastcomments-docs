[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Com o FastComments, todo o texto no widget de comentários é personalizável.

Você pode substituir um único trecho de texto, como o botão de envio, ou todo o texto no widget de comentários inteiro.

Por padrão, o texto no widget de comentários é traduzido com base no locale do usuário. No entanto, podemos substituir o texto, se estivermos confiantes de que nossa base de usuários está usando o mesmo local/idioma, por exemplo:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Texto Personalizado'; code-example-end]

Todas as traduções personalizáveis podem ser encontradas <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">aqui</a> sob a aba "opções avançadas".

No entanto, há uma maneira mais fácil, via a interface de personalização do widget. Lá, podemos simplesmente encontrar o texto que aparece no widget de comentários no locale EN_US e especificar uma substituição.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Painel de texto personalizado com uma string do widget selecionada no menu suspenso e um campo de texto de substituição'; title='Texto Personalizado' app-screenshot-end]

Todas as substituições de traduções atualmente afetam todos os locales.

---