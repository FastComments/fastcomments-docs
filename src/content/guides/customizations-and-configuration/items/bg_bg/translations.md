[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

С FastComments целият текст в уиджет‑а за коментари е персонализируем.

Можете да замените отделен елемент от текста, като бутона за изпращане, или целия текст в уиджет‑а за коментари.

По подразбиране текстът в уиджет‑а за коментари се превежда според локала на потребителя. Въпреки това можем да заменим текста, ако сме уверени, че нашата потребителска база използва същия локал/език, например:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Персонализиран текст'; code-example-end]

Всички персонализируеми преводи могат да се намерят <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">тук</a> под раздела „разширени опции“ таб.

Въпреки това има по‑лесен начин, чрез UI‑то за персонализиране на уиджет‑а. Там можем просто да намерим текста, който се показва в уиджет‑а за коментари в локала EN_US, и да зададем замяна.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Панел за персонализиран текст с избран низ от уиджет‑а от падащото меню и поле за заместващ текст'; title='Персонализиран текст' app-screenshot-end]

Всички замени на преводи в момента засягат всички локали.

Това обхваща текста на самия уиджет. За да преведете коментарите, които вашите посетители пишат, вижте [Comment Translation](#comment-translation).