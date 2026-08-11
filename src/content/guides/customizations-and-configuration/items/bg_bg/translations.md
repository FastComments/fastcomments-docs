[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

С FastComments целият текст в уиджетa за коментари е персонализируем.

Можете да замените отделен елемент от текста, като бутона за изпращане, или целия текст в целия уиджет за коментари.

По подразбиране текстът в уиджетa за коментари се превежда според локала на потребителя. Въпреки това можем да заменим текста, ако сме уверени, че нашата потребителска база използва същия локал/език, например:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Всички персонализируеми преводи могат да се намерят <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">тук</a> под таба „разширени опции“.

Въпреки това има по‑лесен начин, чрез UI за персонализиране на уиджета. Там можем просто да намерим текста, който се показва в уиджета за коментари в локала EN_US, и да зададем замяна.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Панел за персонализиран текст с низ от уиджета, избран от падащото меню, и поле за заместващ текст'; title='Персонализиран текст' app-screenshot-end]

Всички замени на преводи в момента засягат всички локали.