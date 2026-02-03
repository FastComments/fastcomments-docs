FastComments ви позволява да изисквате от първите коментиращи да приемат Условията за ползване преди да изпратят коментар.

Когато е активирано:
- **Анонимните потребители** ще виждат квадратче за потвърждение на Условията за ползване при всяко коментиране
- **Удостоверените потребители** ще виждат отметката само при първия си коментар или когато актуализирате Условията за ползване

### Enabling Terms of Service

Отидете в страницата за персонализиране на уиджета и активирайте квадратчето "Require Terms of Service acceptance":

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

По подразбиране, квадратчето показва "I agree to the Terms of Service and Privacy Policy" с връзки към двата документа. Можете да персонализирате този текст за всяка локализация, ако е необходимо:

1. Изберете "Customize text per locale"
2. Изберете локализация от падащото меню и въведете своя персонализиран текст

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

Когато актуализирате Условията за ползване, задайте датата "Last Updated". Потребителите, които са приели Условията преди тази дата, ще трябва да ги приемат отново:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- Отметката за приемане на Условията се записва с времева маркировка за всеки потребител и за всеки коментар
- Когато потребител приеме Условията за ползване, датата се записва в неговия потребителски профил (за всеки наемател)
- Ако зададете дата "Last Updated", която е след датата на приемане от потребителя, те ще трябва да приемат отново
- За анонимните потребители, които не могат да бъдат проследени, квадратчето се показва при всяко подаване на коментар