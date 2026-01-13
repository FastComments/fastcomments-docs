---
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

Са FastComments-ом, сав текст у видгету за коментаре је прилагодљив.

Можете заменити појединачан део текста, као што је дугме за слање, или сав текст у целом видгету за коментаре.

По подразумевању, текст у видгету за коментаре се преводи у складу са локалом корисника. Међутим, можемо заменити текст ако смо сигурни
да наша база корисника користи исти локал/језик, на пример:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Све прилагодљиве преводе можете наћи <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">овдје</a> под картицом "напредне опције".

Међутим, постоји једноставнији начин, преко корисничког интерфејса за прилагођавање видгета. Тамо можемо једноставно пронаћи текст који се приказује у видгету за коментаре за EN_US локал, и назначити
замјену.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Све замјене превода тренутно се односе на све локале.

---