[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

Коментування можна заблокувати так, щоб не можна було залишати нові коментарі або голоси, встановивши прапорець readonly у true.

Також коментарі не можна буде редагувати або видаляти.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

Це можна налаштувати без коду на сторінці налаштування віджета для цілого домену або сторінки:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## Оновлення!

Починаючи з листопада 2022 року, теми можна блокувати або розблоковувати адміністраторам та модераторам **в реальному часі** через меню з трьома крапками над областю відповіді.

Це заборонить додавання нових коментарів, але при цьому дозволятиме голосування та надаватиме користувачам можливість видаляти свої коментарі, якщо вони цього бажають, тоді як `readonly` цього не дозволяє. 

Це відповідає полю `isClosed` в API `Page`.