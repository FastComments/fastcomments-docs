---
[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments terá comentários ao vivo habilitados.

Isso significa que cada visualizador da sequência de comentários deve ver o mesmo conteúdo.

Por exemplo, se um comentário for adicionado, esse comentário deve ser exibido. Se um comentário for editado ou removido,
então esses comentários serão editados ou removidos para todos os visualizadores da sequência. O mesmo vale para votos e todas as ações de moderação.

No entanto, podemos desativar isso:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Desativar Comentários ao Vivo".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Seção "Desativar Comentários ao Vivo" da página de personalização do widget, desativando atualizações em tempo real da sequência de comentários'; title='Desativar Comentários ao Vivo' app-screenshot-end]

---