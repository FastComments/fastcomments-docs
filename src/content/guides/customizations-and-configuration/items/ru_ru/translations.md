[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

С FastComments весь текст в виджете комментариев настраиваемый.

Вы можете переопределить отдельный фрагмент текста, например кнопку отправки, или весь текст во всём виджете комментариев.

По умолчанию текст в виджете комментариев переводится в соответствии с локалью пользователя. Однако мы можем переопределить текст, если уверены,
что наша пользовательская база использует одну и ту же локаль/язык, например:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Пользовательский текст'; code-example-end]

Все настраиваемые переводы можно найти <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">здесь</a> во вкладке «расширенные параметры» tab.

Однако есть более простой способ, через пользовательский интерфейс настройки виджета. В нем мы можем просто найти текст, который отображается в виджете комментариев в локали EN_US, и указать
замену.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; alt='Панель пользовательского текста с выбранной из выпадающего списка строкой виджета и полем для замены текста'; title='Пользовательский текст' app-screenshot-end]

Все переопределения переводов в настоящее время влияют на все локали.

---