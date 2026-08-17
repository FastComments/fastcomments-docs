---
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Са FastComments, сав текст у виџету за коментаре је прилагодљив.

Можете заменити појединачни део текста, као што је дугме за слање, или сав текст у читавом виџету за коментаре.

Подразумевано, текст у виџету за коментаре се преводи у складу са локалом корисника. Међутим, можемо заменити текст ако смо сигурни да наша база корисника користи исти локал/језик, на пример:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Прилагођени текст'; code-example-end]

Све прилагодљиве преводе можете пронаћи <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">овде</a> под картицом „напредне опције“.

Међутим, постоји једноставнији начин, преко корисничког интерфејса за прилагођавање виџета. Тамо можемо једноставно пронаћи текст који се приказује у виџету за коментаре у EN_US локалу и навести замену.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Панел за прилагођени текст са низом виџета изабраним из падајућег менија и пољем за заменски текст'; title='Прилагођени текст' app-screenshot-end]

Све заменe превода тренутно утичу на све локале.

---