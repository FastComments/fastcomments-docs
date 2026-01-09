[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Са FastComments-ом, сав текст у коментарском видџету је прилагодљив.

Можете заменити појединачни део текста, као што је дугме за слање, или сав текст у целом коментарском видџету.

По подразумевању, текст у коментарском видџету се преводи на основу локала корисника. Међутим, можемо заменити текст ако смо сигурни да наша база корисника користи исти локал/језик, на пример:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Сви прилагодљиви преводи се могу наћи <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">овде</a> под картицом "advanced options".

Међутим, постоји једноставнији начин, преко корисничког интерфејса за прилагођавање видџета. Тамо можемо једноставно пронаћи текст који се приказује у коментарском видџету за EN_US локал и назначити замену.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Све измене превода тренутно утичу на све локале.

---