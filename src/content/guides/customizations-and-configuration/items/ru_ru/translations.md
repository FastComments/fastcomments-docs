---
[related-parameter-start name = 'translations'; type = 'Record<string, string>'; related-parameter-end]

В FastComments весь текст в виджете комментариев настраивается.

Вы можете переопределить отдельный фрагмент текста, например кнопку отправки, или весь текст во всём виджете комментариев.

По умолчанию текст в виджете комментариев переводится в соответствии с локалью пользователя. Однако мы можем переопределить текст, если уверены
что наша аудитория использует ту же локаль/язык, например:

[code-example-start config = {translations: { SUBMIT_REPLY: 'Reply' } }; linesToHighlight = [6, 7, 8]; title = 'Custom Text'; code-example-end]

Все настраиваемые переводы можно найти <a href="https://fastcomments.com/auth/my-account/get-acct-code#translations" target="_blank">здесь</a> на вкладке "дополнительные параметры".

Однако есть более простой способ, через интерфейс настройки виджета. Там мы можем просто найти текст, который отображается в виджете комментариев для локали EN_US, и указать
замену.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-custom-text-option', '#custom-text-options .dropdown-btn', '.dropdown-items a[data-value="JUST_NOW"]']; selector = '#custom-text-options'; title='Custom Text' app-screenshot-end]

Все переопределения переводов в настоящее время применяются ко всем локалям.

---