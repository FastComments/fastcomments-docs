[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments terá comentários em tempo real ativados.

Isso significa que todo visualizador do tópico de comentários deve ver o mesmo conteúdo.

Por exemplo, se um comentário for adicionado, esse comentário deverá aparecer. Se um comentário for editado ou removido,
então esses comentários serão editados ou removidos para todos os visualizadores do tópico. O mesmo vale para votos e todas as ações de moderação.

No entanto, podemos desativar isso:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Desativar comentários em tempo real".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---