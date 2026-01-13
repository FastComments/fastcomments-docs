[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

С FastComments целият текст в уиджета за коментари може да бъде персонализиран.

Можете да замените един елемент от текста, като бутона за изпращане, или целия текст във въпросния уиджeт за коментари.

По подразбиране текстът в уиджета за коментари се превежда въз основа на локала на потребителя. Въпреки това можем да пренапишем текста, ако сме уверени
че нашата база потребители използва един и същ локал/език, например:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Всички персонализирани преводи могат да бъдат намерени <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">тук</a> под раздела "разширени опции".

Има обаче по-лесен начин, чрез UI за персонализиране на уиджета. Там просто можем да намерим текста, който се показва в уиджета за коментари в локала EN_US, и да зададем
замяна.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

В момента всички замествания на преводи засягат всички локали.